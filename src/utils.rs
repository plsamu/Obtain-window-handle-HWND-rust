extern crate winapi;

use regex::Regex;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    EnumWindows, GetWindowTextA, GetWindowTextLengthA, GetWindowThreadProcessId,
};

use crate::window_manager::{WindowInfo, WindowManager};

use std::io::Write;

pub fn get_telegram_windows() -> Vec<WindowInfo> {
    let window_manager = WindowManager {
        windows_info: vec![],
    };
    let window_manager_ref = &window_manager as *const _ as LPARAM;

    unsafe {
        EnumWindows(Some(enum_windows_proc), window_manager_ref as LPARAM);
    }

    // Create a case-insensitive regex pattern that matches "telegram"
    let re = Regex::new(r"(?i)telegram").unwrap();
    let mut output_windows_info: Vec<WindowInfo> = vec![];
    for window_info in window_manager.windows_info {
        // hwnd can be printed as usize but in reality is an opaque pointer, @see src/window_manager.rs
        if re.is_match(&window_info.window_name) {
            output_windows_info.push(window_info);
        }
    }
    output_windows_info
}

pub fn get_telegram_process_ids(windows_info: &mut Vec<WindowInfo>) {
    for window_info in windows_info {
        let mut process_id_tmp: winapi::shared::minwindef::DWORD = 0;
        unsafe { GetWindowThreadProcessId(window_info.window_handle, &mut process_id_tmp) };
        window_info.process_id = Some(process_id_tmp);
    }
}

pub fn print_vec_window_info(windows_info: &Vec<WindowInfo>) {
    for WindowInfo {
        window_name,
        window_handle: _,
        process_id,
    } in windows_info
    {
        let result = match process_id {
            Some(value) => value,
            None => &0,
        };
        println!("Process ID: {}, Name: {}", result, window_name)
    }
}

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
    let window_manager = &mut *(lparam as *mut WindowManager);

    let length = GetWindowTextLengthA(hwnd) as usize;
    let mut window_name_space = vec![0i8; length + 1];
    GetWindowTextA(
        hwnd,
        window_name_space.as_mut_ptr(),
        window_name_space.len().try_into().unwrap(),
    );

    let window_name: String = window_name_space
        .iter()
        .map(|&byte| byte as u8) // Cast to u8 since chars are UTF-8 encoded.
        .filter(|&byte| byte.is_ascii()) // Filter out non-ASCII characters if necessary.
        .map(|byte| byte as char) // Convert the u8 to a char.
        .collect();

    window_manager.add_window_name(window_name, hwnd);
    1
}

#[allow(dead_code)]
pub fn print_to_file_in_base64(filename: String, content: Vec<String>) {
    let mut file = std::fs::OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .write(true)
        .append(true) // Append to the file if it exists
        .open(filename)
        .expect("unable to open file");

    for s in content {
        let to_save = base64::encode(s) + "\n";
        file.write_all(to_save.as_bytes())
            .expect("Unable to write to file");
    }
}

#[allow(dead_code)]
pub fn check_if_vec_contain_empty_and_human_readable_elements(mut content: Vec<String>) {
    content.retain(|s| {
        let is_utf8 = std::str::from_utf8(s.as_bytes()).is_ok();
        is_utf8 && !s.is_empty()
    });
}
