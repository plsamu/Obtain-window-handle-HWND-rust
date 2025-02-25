use winapi::shared::{minwindef::DWORD, windef::HWND};

/**
 * https://stackoverflow.com/questions/53219422/what-data-type-does-hwnd-type-belong-to
 */
pub struct WindowInfo {
    pub window_name: String,
    pub window_handle: HWND,
    pub process_id: Option<DWORD>,
}

pub struct WindowManager {
    pub windows_info: Vec<WindowInfo>,
}

impl WindowManager {
    pub fn add_window_name(&mut self, name: String, handle_window: HWND) {
        self.windows_info.push(WindowInfo {
            window_name: name,
            window_handle: handle_window,
            process_id: None,
        });
    }
}
