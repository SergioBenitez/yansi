#[cfg(windows)]
mod windows_console {
    use std::os::raw::c_void;

    #[allow(non_camel_case_types)] type c_ulong = u32;
    #[allow(non_camel_case_types)] type c_int = i32;
    type DWORD = c_ulong;
    type LPDWORD = *mut DWORD;
    type HANDLE = *mut c_void;
    type BOOL = c_int;

    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
    const STD_OUTPUT_HANDLE: DWORD = 0xFFFFFFF5;
    const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
    const FALSE: BOOL = 0;
    const TRUE: BOOL = 1;

    // This is the win32 console API, taken from the 'winapi' crate.
    extern "system" {
        fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
        fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
        fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
    }

    pub fn enable_ascii_colors() -> bool {
        unsafe {
            let stdout_handle: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
            if stdout_handle == INVALID_HANDLE_VALUE {
                return false
            }

            let mut dw_mode: DWORD = 0;
            if GetConsoleMode(stdout_handle, &mut dw_mode) == FALSE {
                return false
            }

            dw_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            SetConsoleMode(stdout_handle, dw_mode) == TRUE
        }
    }
}

#[cfg(not(windows))]
mod windows_console {
    pub fn enable_ascii_colors() -> bool { true }
}

pub use self::windows_console::enable_ascii_colors;
