// Real x86_64 long-mode boot harness: Multiboot2 entry (CPU starts in
// 32-bit protected mode per the Multiboot spec, regardless of the
// kernel ELF's own bitness) sets up identity-mapped page tables,
// enables PAE + long mode (EFER.LME) + paging, far-jumps into a 64-bit
// code segment, then hands off to genuine 64-bit Rust which exercises a
// real workspace type (`kernel_types::SafePageFrame`) before shutting
// QEMU down cleanly via the isa-debug-exit device.
#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

global_asm!(
    r#"
.section .multiboot2, "a"
.align 8
mb2_header_start:
    .long 0xE85250D6
    .long 0
    .long mb2_header_end - mb2_header_start
    .long -(0xE85250D6 + 0 + (mb2_header_end - mb2_header_start))
    .align 8
    .word 0
    .word 0
    .long 8
mb2_header_end:

.section .bss
.align 4096
pml4_table:
    .skip 4096
pdpt_table:
    .skip 4096
pd_table:
    .skip 4096
.align 16
stack_bottom:
    .skip 16384
stack_top:

.section .rodata
.align 8
gdt64:
    .quad 0
    .quad 0x00AF9A000000FFFF
    .quad 0x00AF92000000FFFF
gdt64_end:
gdt64_ptr:
    .word gdt64_end - gdt64 - 1
    .quad gdt64

.section .text
.code32
.global _start
_start:
    cli
    mov esp, offset stack_top

    # PML4[0] = &pdpt_table | PRESENT|WRITABLE
    mov eax, offset pdpt_table
    or eax, 3
    mov dword ptr [pml4_table], eax
    mov dword ptr [pml4_table+4], 0

    # PDPT[0] = &pd_table | PRESENT|WRITABLE
    mov eax, offset pd_table
    or eax, 3
    mov dword ptr [pdpt_table], eax
    mov dword ptr [pdpt_table+4], 0

    # Fill PD: 512 x 2MiB pages, identity-mapped, present|writable|huge
    mov edi, offset pd_table
    xor ebx, ebx
    mov ecx, 512
fill_pd:
    mov eax, ebx
    or eax, 0x83
    mov dword ptr [edi], eax
    mov dword ptr [edi+4], 0
    add ebx, 0x200000
    add edi, 8
    loop fill_pd

    mov eax, offset pml4_table
    mov cr3, eax

    mov eax, cr4
    or eax, 0x20
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 0x100
    wrmsr

    mov eax, cr0
    or eax, 0x80000000
    mov cr0, eax

    lgdt [gdt64_ptr]

    ljmp 0x08, offset long_mode_entry

.code64
long_mode_entry:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov rsp, offset stack_top
    xor rbp, rbp
    and rsp, -16

    call kmain

    cli
2:
    hlt
    jmp 2b
"#
);

fn outb(port: u16, val: u8) {
    // SAFETY: `out` to an I/O port is only observable by hardware (the
    // 16550 UART / QEMU debug-exit device), touches no Rust-managed
    // memory, and `options(nomem, nostack, preserves_flags)` accurately
    // describes its effects, so it cannot violate any Rust aliasing or
    // memory-safety invariant.
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
    }
}

fn serial_print(text: &str) {
    for byte in text.bytes() {
        outb(0x3F8, byte);
    }
}

/// QEMU's isa-debug-exit device: exit code = (value << 1) | 1.
/// value=0x00 -> exit code 1 (success sentinel used by this harness).
/// value=0x11 -> exit code 35 (panic sentinel).
fn qemu_exit(value: u8) -> ! {
    outb(0xF4, value);
    loop {
        // SAFETY: `hlt` only stops the CPU until the next interrupt; it
        // takes no operands, touches no memory, and this loop is
        // reachable only after the isa-debug-exit write above, which
        // terminates QEMU before `hlt` is ever actually needed to run.
        unsafe { asm!("hlt") };
    }
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    serial_print("[INFO] Booting RunuX x86_64 QEMU Harness (long mode)...\n");
    serial_print("[INFO] PAE + EFER.LME + CR0.PG enabled, identity-mapped first 1GiB\n");

    // Exercise a real workspace type, not just print statements: the
    // RAII newtype wrapper AGENTS.md requires for page-frame handling
    // (invariant #2: "SafePageFrame"). This is genuine kernel_types
    // code, not a stub -- both the null-rejection and the valid-pointer
    // path run for real here.
    let mut sample: u64 = 0xC0FFEE;
    let ptr = &mut sample as *mut u64 as *mut core::ffi::c_void;

    match kernel_types::SafePageFrame::new(ptr, 0) {
        Some(_frame) => serial_print("[PASS] SafePageFrame::new(valid_ptr) -> Some\n"),
        None => qemu_exit(0x11),
    }

    match kernel_types::SafePageFrame::new(core::ptr::null_mut(), 0) {
        None => serial_print("[PASS] SafePageFrame::new(null) -> None (rejected as required)\n"),
        Some(_) => qemu_exit(0x11),
    }

    serial_print("[SUCCESS] RunuX x86_64 booted successfully inside QEMU (long mode)!\n");
    qemu_exit(0x00)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_print("\n[ERROR] KERNEL PANIC!\n");
    qemu_exit(0x11)
}
