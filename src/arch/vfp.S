.macro fpstate_save
    // save fp/neon context
    mrs     x9, fpcr
    mrs     x10, fpsr
    stp     q0, q1, [x0, 0 * 16]
    stp     q2, q3, [x0, 2 * 16]
    stp     q4, q5, [x0, 4 * 16]
    stp     q6, q7, [x0, 6 * 16]
    stp     q8, q9, [x0, 8 * 16]
    stp     q10, q11, [x0, 10 * 16]
    stp     q12, q13, [x0, 12 * 16]
    stp     q14, q15, [x0, 14 * 16]
    stp     q16, q17, [x0, 16 * 16]
    stp     q18, q19, [x0, 18 * 16]
    stp     q20, q21, [x0, 20 * 16]
    stp     q22, q23, [x0, 22 * 16]
    stp     q24, q25, [x0, 24 * 16]
    stp     q26, q27, [x0, 26 * 16]
    stp     q28, q29, [x0, 28 * 16]
    stp     q30, q31, [x0, 30 * 16]
    str     x9, [x0, 64 *  8]
    str     x10, [x0, 65 * 8]
.endm

.macro fpstate_restore
    // restore fp/neon context
    ldp     q0, q1, [x1, 0 * 16]
    ldp     q2, q3, [x1, 2 * 16]
    ldp     q4, q5, [x1, 4 * 16]
    ldp     q6, q7, [x1, 6 * 16]
    ldp     q8, q9, [x1, 8 * 16]
    ldp     q10, q11, [x1, 10 * 16]
    ldp     q12, q13, [x1, 12 * 16]
    ldp     q14, q15, [x1, 14 * 16]
    ldp     q16, q17, [x1, 16 * 16]
    ldp     q18, q19, [x1, 18 * 16]
    ldp     q20, q21, [x1, 20 * 16]
    ldp     q22, q23, [x1, 22 * 16]
    ldp     q24, q25, [x1, 24 * 16]
    ldp     q26, q27, [x1, 26 * 16]
    ldp     q28, q29, [x1, 28 * 16]
    ldp     q30, q31, [x1, 30 * 16]
    ldr     x9, [x1, 64 * 8]
    ldr     x10, [x1, 65 * 8]
    msr     fpcr, x9
    msr     fpsr, x10
.endm

.section .text
.global fpstate_switch
fpstate_switch: 
    fpstate_save 
    fpstate_restore
    isb
    ret
