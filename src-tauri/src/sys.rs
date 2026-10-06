#[cfg(target_os = "windows")]
use windows::{
    core::{w, BOOL},
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, FindWindowExW, FindWindowW, SendMessageTimeoutW, SetParent, SMTO_NORMAL,
        },
    },
};

/// Pins the given window underneath the desktop icons by re-parenting it
/// into the WorkerW window that sits between the wallpaper and the icons.
#[cfg(target_os = "windows")]
pub fn attach_to_desktop(hwnd: HWND) {
    unsafe {
        let progman = FindWindowW(w!("Progman"), None).unwrap_or_default();

        if !progman.is_invalid() {
            // 0x052C asks Progman to spawn a WorkerW behind the desktop icons.
            let _ = SendMessageTimeoutW(
                progman,
                0x052C,
                WPARAM::default(),
                LPARAM::default(),
                SMTO_NORMAL,
                1000,
                None,
            );
        }

        // The WorkerW handle is written through the LPARAM pointer (no global state).
        let mut worker_w = HWND::default();

        unsafe extern "system" fn enum_windows_proc(tophandle: HWND, lparam: LPARAM) -> BOOL {
            let def_view =
                FindWindowExW(Some(tophandle), None, w!("SHELLDLL_DefView"), None).unwrap_or_default();
            if !def_view.is_invalid() {
                // The wallpaper WorkerW is the next sibling after the one hosting SHELLDLL_DefView.
                let out = lparam.0 as *mut HWND;
                *out = FindWindowExW(None, Some(tophandle), w!("WorkerW"), None).unwrap_or_default();
            }
            BOOL(1)
        }

        let _ = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut worker_w as *mut HWND as isize),
        );

        // Windows 11 24H2+: no separate WorkerW is spawned, fall back to Progman itself.
        let parent = if !worker_w.is_invalid() { worker_w } else { progman };
        if !parent.is_invalid() {
            let _ = SetParent(hwnd, Some(parent));
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn attach_to_desktop<T>(_window_hwnd: T) {
    // macOS/Linux implementation or no-op
}
