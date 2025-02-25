mod utils;
mod window_manager;
extern crate winapi;

use utils::{get_telegram_process_ids, get_telegram_windows, print_vec_window_info};

fn main() {
    let mut telegram_windows = get_telegram_windows();
    get_telegram_process_ids(&mut telegram_windows);
    print_vec_window_info(&telegram_windows);
}
