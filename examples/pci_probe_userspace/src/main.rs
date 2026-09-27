//! Runs RunuX's real `driver_pci_probe::pci_enumerate` -- the exact same
//! function `examples/x86_64_qemu_harness` calls at boot -- as a
//! privileged Linux userspace binary instead of inside a custom kernel.
//!
//! Why this exists: verifying the bare-metal PCI code against real
//! (non-QEMU) hardware would normally need a full custom kernel boot on
//! a cloud VM, which needs bootloader/serial-console plumbing that is
//! separate work. Linux lets a root process call `iopl(3)` to grant
//! itself the same raw port-I/O access ring 0 code has, so the same
//! `in`/`out`-instruction-based config-space reads this project's
//! kernel code performs can run here, unmodified, against genuine
//! hardware -- a real cross-check, not a simulation of one.
//!
//! Requires root (`CAP_SYS_RAWIO`). Not meant to run outside a
//! disposable VM: `iopl(3)` grants this process (and anything it execs)
//! unrestricted hardware I/O port access for its lifetime.

fn main() {
    // SAFETY: iopl(3) is a well-defined Linux syscall; failure is
    // reported via errno and checked below. This process must be root.
    let rc = unsafe { libc::iopl(3) };
    if rc != 0 {
        eprintln!(
            "iopl(3) failed (errno {}) -- must run as root.",
            std::io::Error::last_os_error()
        );
        std::process::exit(1);
    }

    let mut devices = [driver_pci_core::PciDeviceInfo {
        address: driver_pci_core::PciAddress { bus: 0, device: 0, function: 0 },
        vendor_id: 0,
        device_id: 0,
        class: 0,
        subclass: 0,
        header_type: 0,
        multi_function: false,
    }; driver_pci_probe::MAX_ENUMERATED_DEVICES];

    // SAFETY: iopl(3) above granted this process full I/O port access,
    // satisfying pci_enumerate's precondition; called once, synchronously.
    let count = unsafe { driver_pci_probe::pci_enumerate(&mut devices) };

    println!("RunuX driver_pci_probe::pci_enumerate found {count} real PCI device(s):");
    for dev in &devices[..count] {
        print!(
            "  {:02x}:{:02x}.{:x} vendor={:04x} device={:04x} class={:02x} subclass={:02x}",
            dev.address.bus, dev.address.device, dev.address.function,
            dev.vendor_id, dev.device_id, dev.class, dev.subclass,
        );
        if let Some(name) = driver_pci_core::known_gpu_name(dev.vendor_id, dev.device_id) {
            print!(" -- {name}");
        } else if dev.vendor_id == driver_pci_core::GOOGLE_VENDOR_ID {
            print!(" -- Google, Inc. device (see lspci -nn for the specific accelerator/NIC name)");
        }
        println!();

        if dev.is_display_controller() || driver_pci_core::known_gpu_name(dev.vendor_id, dev.device_id).is_some() {
            for (i, bar_offset) in [0x10u8, 0x14, 0x18, 0x1C, 0x20, 0x24].iter().enumerate() {
                // SAFETY: same iopl(3) grant as above.
                let size = unsafe {
                    driver_pci_access::pci_bar_size(dev.address.bus, dev.address.device, dev.address.function, *bar_offset)
                };
                if size > 0 {
                    println!("    BAR{i} size=0x{size:08x} bytes");
                }
            }
        }
    }
}
