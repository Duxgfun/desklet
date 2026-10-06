#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM, BOOL},
    UI::WindowsAndMessaging::{
        EnumWindows, FindWindowExW, FindWindowW, SendMessageTimeoutW, SetParent, SMTO_NORMAL,
    },
};

#[cfg(target_os = "windows")]
pub fn attach_to_desktop(window_hwnd: *mut std::ffi::c_void) {
    let hwnd = HWND(window_hwnd as _);
    unsafe {
        let progman = FindWindowW(windows::core::w!("Progman"), None).unwrap_or(HWND::default());
        
        if progman != HWND::default() {
            SendMessageTimeoutW(
                progman,
                0x052C,
                WPARAM::default(),
                LPARAM::default(),
                SMTO_NORMAL,
                1000,
                None,
            );
        }

        static mut WORKER_W: HWND = HWND(0 as _);

        unsafe extern "system" fn enum_windows_proc(tophandle: HWND, _: LPARAM) -> BOOL {
            let p = FindWindowExW(tophandle, None, windows::core::w!("SHELLDLL_DefView"), None).unwrap_or(HWND::default());
            if p != HWND::default() {
                // Gets the WorkerW Window after the current one.
                WORKER_W = FindWindowExW(None, tophandle, windows::core::w!("WorkerW"), None).unwrap_or(HWND::default());
            }
            BOOL(1)
        }

        EnumWindows(Some(enum_windows_proc), LPARAM::default()).ok();

        if WORKER_W != HWND::default() {
            SetParent(hwnd, WORKER_W).ok();
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn attach_to_desktop(_window_hwnd: *mut std::ffi::c_void) {
    // macOS/Linux implementation or no-op
}
