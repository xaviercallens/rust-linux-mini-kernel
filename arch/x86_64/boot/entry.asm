; Rust Linux Mini Kernel - x86_64 Entry Point (NASM syntax)
; This is called by the bootloader (multiboot2 or direct boot)

BITS 64
section .text
global _start
global x86_out8
global x86_in8

extern start_kernel
extern __bss_start
extern __bss_end

_start:
    ; Disable interrupts
    cli

    ; Set up stack
    ; Stack grows downward, so set rsp to top of stack area
    mov rsp, stack_top

    ; Clear frame pointer for clean backtraces
    xor rbp, rbp

    ; Align stack to 16 bytes (required by System V ABI)
    and rsp, -16

    ; Zero out BSS section
    mov rdi, __bss_start
    mov rcx, __bss_end
    sub rcx, rdi              ; Calculate size
    xor al, al                ; Zero byte
    rep stosb                 ; Fill BSS with zeros

    ; Call Rust kernel entry point
    ; This will never return
    call start_kernel

    ; If somehow we return, halt forever
.halt_loop:
    cli
    hlt
    jmp .halt_loop

; x86 Port I/O functions for Rust

; void x86_out8(u16 port, u8 value)
; rdi = port, rsi = value
x86_out8:
    mov dx, di          ; port to dx (u16)
    mov al, sil         ; value to al (u8)
    out dx, al          ; output byte
    ret

; u8 x86_in8(u16 port)
; rdi = port, returns value in al
x86_in8:
    mov dx, di          ; port to dx (u16)
    in al, dx           ; input byte to al
    ret

; Kernel stack
section .bss
align 16
stack_bottom:
    resb 16384          ; 16 KB stack
stack_top:
