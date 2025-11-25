.section .text._start
.global _start
.type _start, %function

_start:
    /* Disable interrupts */
    msr DAIFSet, #0xf

    /* Simple stack */
    adr x30, _start
    mov sp, x30

    /* Clear BSS – required for Rust */
    ldr x0, =__bss_start
    ldr x1, =__bss_end
    b   2f
1:  str xzr, [x0], #8
2:  cmp x0, x1
    b.lt 1b

    bl _start_rust
    b   .

.global _start_rust
