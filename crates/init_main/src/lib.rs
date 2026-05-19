#![cfg_attr(not(test), no_std)]
//! start_kernel() entry point - Rust Linux Mini Kernel

#[allow(non_camel_case_types)]
type c_int = i32;

#[cfg(not(test))]
mod memory;

extern "C" {
    fn printk_init() -> c_int;
    fn printk_str(s: *const u8, len: usize);
    fn arch_setup_init() -> c_int;
    #[cfg(not(test))]
    fn page_alloc_init() -> c_int;
    #[cfg(not(test))]
    fn slab_init() -> c_int;
}

#[inline]
unsafe fn print(msg: &[u8]) {
    printk_str(msg.as_ptr(), msg.len());
}

/// Kernel entry point - never returns.
///
/// Initializes the kernel subsystems and enters the idle loop.
///
/// # Safety
///
/// - Must be called from assembly boot code
/// - Requires valid stack setup
/// - Assumes interrupts are disabled
/// - Calls other unsafe initialization functions
#[no_mangle]
pub unsafe extern "C" fn start_kernel() -> ! {
    printk_init();
    print(b"Rust Linux Mini Kernel v9.0.0-phase2 booting...\n");

    if arch_setup_init() != 0 {
        print(b"PANIC: arch_setup_init failed\n");
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        loop { core::arch::asm!("hlt", options(nomem, nostack)); }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        panic!("arch_setup_init failed");
    }

    print(b"Architecture initialized\n");

    // Phase 2: Initialize memory management
    #[cfg(not(test))]
    {
        if page_alloc_init() != 0 {
            print(b"PANIC: page_alloc_init failed\n");
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            loop { core::arch::asm!("hlt", options(nomem, nostack)); }
            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
            panic!("page_alloc_init failed");
        }
        print(b"Page allocator initialized\n");

        if slab_init() != 0 {
            print(b"PANIC: slab_init failed\n");
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            loop { core::arch::asm!("hlt", options(nomem, nostack)); }
            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
            panic!("slab_init failed");
        }
        print(b"SLAB allocator initialized\n");
    }

    print(b"Kernel panic - Phase 2 boot complete!\n");

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    loop { core::arch::asm!("hlt", options(nomem, nostack)); }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    panic!("Kernel boot complete");
}

/// Initialize the init_main subsystem.
///
/// # Safety
///
/// - Currently a no-op, returns success
#[no_mangle]
pub unsafe extern "C" fn init_main_init() -> c_int { 0 }

/// Cleanup function for init_main subsystem.
///
/// # Safety
///
/// - Currently a no-op, safe to call at any time
#[no_mangle]
pub unsafe extern "C" fn init_main_exit() {}

#[no_mangle]
pub static mut INIT_MAIN_INITIALIZED: bool = false;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mock state for tracking function calls
    static MOCK_STATE: Mutex<MockState> = Mutex::new(MockState::new());

    #[derive(Debug)]
    struct MockState {
        printk_init_called: bool,
        printk_str_calls: Vec<Vec<u8>>,
        arch_setup_result: c_int,
    }

    impl MockState {
        const fn new() -> Self {
            Self {
                printk_init_called: false,
                printk_str_calls: Vec::new(),
                arch_setup_result: 0,
            }
        }

        fn reset(&mut self) {
            self.printk_init_called = false;
            self.printk_str_calls.clear();
            self.arch_setup_result = 0;
        }

        fn set_arch_setup_result(&mut self, result: c_int) {
            self.arch_setup_result = result;
        }
    }

    // Mock implementations
    #[no_mangle]
    pub unsafe extern "C" fn printk_init() -> c_int {
        MOCK_STATE.lock().unwrap().printk_init_called = true;
        0
    }

    #[no_mangle]
    pub unsafe extern "C" fn printk_str(s: *const u8, len: usize) {
        if !s.is_null() && len > 0 {
            let slice = core::slice::from_raw_parts(s, len);
            MOCK_STATE.lock().unwrap().printk_str_calls.push(slice.to_vec());
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn arch_setup_init() -> c_int {
        MOCK_STATE.lock().unwrap().arch_setup_result
    }

    #[test]
    fn test_init_main_init() {
        unsafe {
            let result = init_main_init();
            assert_eq!(result, 0, "init_main_init should return 0");
        }
    }

    #[test]
    fn test_init_main_exit_is_safe() {
        unsafe {
            init_main_exit(); // Should not panic
        }
    }

    #[test]
    fn test_print_helper_null() {
        let mut state = MOCK_STATE.lock().unwrap();
        state.reset();
        drop(state);

        unsafe {
            print(b"");
        }

        let state = MOCK_STATE.lock().unwrap();
        assert_eq!(state.printk_str_calls.len(), 0, "Empty slice should not call printk_str");
    }

    #[test]
    fn test_print_helper_valid() {
        let mut state = MOCK_STATE.lock().unwrap();
        state.reset();
        drop(state);

        unsafe {
            print(b"test message");
        }

        let state = MOCK_STATE.lock().unwrap();
        assert_eq!(state.printk_str_calls.len(), 1, "Should call printk_str once");
        assert_eq!(state.printk_str_calls[0], b"test message", "Should pass correct message");
    }

    #[test]
    fn test_print_helper_multiple() {
        let mut state = MOCK_STATE.lock().unwrap();
        state.reset();
        drop(state);

        unsafe {
            print(b"first");
            print(b"second");
            print(b"third");
        }

        let state = MOCK_STATE.lock().unwrap();
        assert_eq!(state.printk_str_calls.len(), 3, "Should call printk_str three times");
        assert_eq!(state.printk_str_calls[0], b"first");
        assert_eq!(state.printk_str_calls[1], b"second");
        assert_eq!(state.printk_str_calls[2], b"third");
    }

    #[test]
    fn test_init_main_initialized_flag() {
        unsafe {
            let initial = INIT_MAIN_INITIALIZED;
            INIT_MAIN_INITIALIZED = true;
            assert!(INIT_MAIN_INITIALIZED);
            INIT_MAIN_INITIALIZED = initial;
        }
    }

    // Note: Cannot directly test start_kernel() as it never returns
    // and enters an infinite loop. Integration tests would be needed.
}
