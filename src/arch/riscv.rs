use core::arch::asm;
use memory_addr::VirtAddr;

#[cfg(not(feature = "async"))]
/// Saved hardware states of a task.
///
/// The context usually includes:
///
/// - Callee-saved registers
/// - Stack pointer register
/// - Thread pointer register (for thread-local storage, currently unsupported)
/// - FP/SIMD registers
///
/// On context switch, current task saves its context from CPU to memory,
/// and the next task restores its context from memory to CPU.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct TaskContext {
    pub ra: usize, // return address (x1)
    pub sp: usize, // stack pointer (x2)

    pub s0: usize, // x8-x9
    pub s1: usize,

    pub s2: usize, // x18-x27
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,

    pub tp: usize,
    // TODO: FP states
}

#[cfg(feature = "async")]
/// Saved hardware states of a task.
///
/// The context usually includes:
///
/// - Callee-saved registers
/// - Stack pointer register
/// - Thread pointer register (for thread-local storage, currently unsupported)
/// - FP/SIMD registers
///
/// On context switch, current task saves its context from CPU to memory,
/// and the next task restores its context from memory to CPU.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct TaskContext {
    pub ra: usize, // return address (x1)
    pub sp: usize, // stack pointer (x2)

    pub s0: usize, // x8-x9
    pub s1: usize,

    pub s2: usize, // x18-x27
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    
    pub tp: usize,
    // TODO: FP states
    pub gp: usize,
    // t
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    // a
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    /// Privilege information.
    pub priv_info: PrivInfo,
}

#[cfg(feature = "async")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub enum PrivInfo {
    SPrivilige(SPrivilige),
    UPrivilige(UPrivilige),
    #[default]
    UnKnown
}

#[cfg(feature = "async")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SPrivilige {
    pub sstatus: usize,
    pub sepc: usize,
    pub stvec: usize,
    pub sie: usize,
}

#[cfg(feature = "async")]
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct UPrivilige {
    pub ustatus: usize,
    pub uepc: usize,
    pub utvec: usize,
    pub uie: usize,
}

impl TaskContext {
    /// Creates a new default context for a new task.
    pub const fn new() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }

    /// Initializes the context for a new task, with the given entry point and
    /// kernel stack.
    pub fn init(&mut self, entry: usize, kstack_top: VirtAddr, tls_area: VirtAddr) {
        self.sp = kstack_top.as_usize();
        self.ra = entry;
        self.tp = tls_area.as_usize();
    }
}

#[cfg(target_arch = "riscv32")]
core::arch::global_asm!(
    r"
.ifndef XLENB
.equ XLENB, 4

.macro LDR rd, rs, off
    lw \rd, \off*XLENB(\rs)
.endm
.macro STR rs2, rs1, off
    sw \rs2, \off*XLENB(\rs1)
.endm

.endif"
);

#[cfg(target_arch = "riscv64")]
core::arch::global_asm!(
    r"
.ifndef XLENB
.equ XLENB, 8

.macro LDR rd, rs, off
    ld \rd, \off*XLENB(\rs)
.endm
.macro STR rs2, rs1, off
    sd \rs2, \off*XLENB(\rs1)
.endm

.endif",
);

#[naked]
/// Switches the context from the current task to the next task.
///
/// # Safety
///
/// This function is unsafe because it directly manipulates the CPU registers.
pub unsafe extern "C" fn context_switch(_current_task: &mut TaskContext, _next_task: &TaskContext) {
    asm!(
        "
        // save old context (callee-saved registers)
        STR     ra, a0, 0
        STR     sp, a0, 1
        STR     s0, a0, 2
        STR     s1, a0, 3
        STR     s2, a0, 4
        STR     s3, a0, 5
        STR     s4, a0, 6
        STR     s5, a0, 7
        STR     s6, a0, 8
        STR     s7, a0, 9
        STR     s8, a0, 10
        STR     s9, a0, 11
        STR     s10, a0, 12
        STR     s11, a0, 13

        // restore new context
        LDR     s11, a1, 13
        LDR     s10, a1, 12
        LDR     s9, a1, 11
        LDR     s8, a1, 10
        LDR     s7, a1, 9
        LDR     s6, a1, 8
        LDR     s5, a1, 7
        LDR     s4, a1, 6
        LDR     s3, a1, 5
        LDR     s2, a1, 4
        LDR     s1, a1, 3
        LDR     s0, a1, 2
        LDR     sp, a1, 1
        LDR     ra, a1, 0

        ret",
        options(noreturn),
    )
}

#[cfg(feature = "async")]
const TASKCONTEXT_SIZE: usize = core::mem::size_of::<TaskContext>();

#[cfg(feature = "async")]
use core::ptr::NonNull;

#[cfg(feature = "async")]
extern "C" {
    fn schedule_with_sp_change();
}

#[cfg(feature = "async")]
#[naked]
// Save the previous context to the stack.
pub unsafe extern "C" fn save_prev_ctx(prev_ctx_ref: &mut NonNull<TaskContext>) {
    core::arch::asm!(
        "
        addi    sp, sp, -{taskctx_size}
        STR     ra, sp, 0
        STR     sp, sp, 1
        STR     s0, sp, 2
        STR     s1, sp, 3
        STR     s2, sp, 4
        STR     s3, sp, 5
        STR     s4, sp, 6
        STR     s5, sp, 7
        STR     s6, sp, 8
        STR     s7, sp, 9
        STR     s8, sp, 10
        STR     s9, sp, 11
        STR     s10, sp, 12
        STR     s11, sp, 13
        STR     tp, sp, 14
        STR     gp, sp, 15
        STR     t0, sp, 16
        STR     t1, sp, 17
        STR     t2, sp, 18
        STR     t3, sp, 19
        STR     t4, sp, 20
        STR     t5, sp, 21
        STR     t6, sp, 22
        STR     a0, sp, 23
        STR     a1, sp, 24
        STR     a2, sp, 25
        STR     a3, sp, 26
        STR     a4, sp, 27
        STR     a5, sp, 28
        STR     a6, sp, 29
        STR     a7, sp, 30
        ",
        // a0 -> ctx_ref
        // sp -> *mut TaskContext
        "STR     sp, a0, 0",
        "call {schedule_with_sp_change}",
        // // The stack has changed, if the next task is a coroutine, the execution will return to here.
        // // But the ra is not correct.
        // "ret",
        taskctx_size = const TASKCONTEXT_SIZE,
        schedule_with_sp_change = sym schedule_with_sp_change,
        options(noreturn),
    )
}

#[cfg(feature = "async")]
#[naked]
/// Load the next context from the stack.
pub unsafe extern "C" fn load_next_ctx(next_ctx_ref: &mut NonNull<TaskContext>) {
    core::arch::asm!(
        "LDR     sp, a0, 0",
        "
        LDR     ra, sp, 0
        LDR     sp, sp, 1
        LDR     s0, sp, 2
        LDR     s1, sp, 3
        LDR     s2, sp, 4
        LDR     s3, sp, 5
        LDR     s4, sp, 6
        LDR     s5, sp, 7
        LDR     s6, sp, 8
        LDR     s7, sp, 9
        LDR     s8, sp, 10
        LDR     s9, sp, 11
        LDR     s10, sp, 12
        LDR     s11, sp, 13
        LDR     tp, sp, 14
        LDR     gp, sp, 15
        LDR     t0, sp, 16
        LDR     t1, sp, 17
        LDR     t2, sp, 18
        LDR     t3, sp, 19
        LDR     t4, sp, 20
        LDR     t5, sp, 21
        LDR     t6, sp, 22
        LDR     a0, sp, 23
        LDR     a1, sp, 24
        LDR     a2, sp, 25
        LDR     a3, sp, 26
        LDR     a4, sp, 27
        LDR     a5, sp, 28
        LDR     a6, sp, 29
        LDR     a7, sp, 30
        addi    sp, sp, {taskctx_size}
        ret",
        taskctx_size = const TASKCONTEXT_SIZE,
        options(noreturn),
    )
}