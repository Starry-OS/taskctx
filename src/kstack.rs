extern crate alloc;
use core::{alloc::Layout, ptr::NonNull};

use memory_addr::VirtAddr;

pub(crate) struct TaskStack {
    ptr: NonNull<u8>,
    layout: Layout,
}

// arch_boot
unsafe extern "C" {
    fn current_boot_stack() -> *mut u8;
}

impl TaskStack {
    pub fn new_init() -> Self {
        let layout = Layout::from_size_align(axconfig::TASK_STACK_SIZE, 16).unwrap();
        unsafe {
            Self {
                ptr: NonNull::new(current_boot_stack()).unwrap(),
                layout,
            }
        }
    }

    pub fn alloc(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 16).unwrap();
        Self {
            ptr: NonNull::new(unsafe { alloc::alloc::alloc(layout) }).unwrap(),
            layout,
        }
    }

    pub const fn top(&self) -> VirtAddr {
        unsafe { core::mem::transmute(self.ptr.as_ptr().add(self.layout.size())) }
    }

    pub const fn down(&self) -> VirtAddr {
        unsafe { core::mem::transmute(self.ptr.as_ptr()) }
    }

    // #[cfg(feature = "monolithic")]
    // /// 获取内核栈第一个压入的trap上下文，防止出现内核trap嵌套
    // pub fn get_first_trap_frame(&self) -> *mut TrapFrame {
    //     (self.top().as_usize() - core::mem::size_of::<TrapFrame>()) as *mut TrapFrame
    // }
}

impl Drop for TaskStack {
    fn drop(&mut self) {
        unsafe { alloc::alloc::dealloc(self.ptr.as_ptr(), self.layout) }
    }
}
