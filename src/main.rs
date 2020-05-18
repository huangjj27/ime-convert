//! Here is a reimplement of [`im-select`] in Rust, Which provides different APIs
//! and only support Windows (MacOS will be supportted in some day).
//!
//! ## Install
//! ```
//! cargo install im-select-rs
//! ```
//!
//! ## Manual
//! This CLI command provides two basic usages.
//! - To get current IME information, just run `im-select`. return whill be in `{IME}.{conversion}` format
//! - To switch to preferred IME and conversion, run like `im-select {IME}.{conversion}`
//!
//! NOTES: these commands will only work on VSCodeVim config.
//!
//! [`im-select`]: https://github.com/daipeihust/im-select

mod ime;

use winapi::shared::minwindef::WORD;
use winapi::shared::minwindef::LOWORD;
use winapi::um::winuser::GetKeyboardLayout;
use winapi::shared::minwindef::DWORD;
use winapi::um::winuser::GetWindowThreadProcessId;
use structopt::StructOpt;
use winapi::shared::minwindef::LPARAM;
use winapi::um::winuser::GetForegroundWindow;
use winapi::um::winuser::PostMessageA;
use winapi::um::winuser::WM_INPUTLANGCHANGEREQUEST;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "im-select-rs",
    about = "Rust version of im-select implemented with winapi"
)]
struct Opt {
    ime: Option<isize>,
}

// const ENGLISH: isize = 1033;

fn get_ime_idx() -> WORD {
    unsafe {
        let hwnd = GetForegroundWindow();
        let threadID = GetWindowThreadProcessId(hwnd, std::ptr::null_mut::<DWORD>());
        let currentLayout = GetKeyboardLayout(threadID);
        LOWORD(currentLayout as DWORD)
    }
}

fn switch_ime_to(idx: isize) {
    unsafe {
        let hwnd = GetForegroundWindow();
        let currentLayout: LPARAM = idx;
        PostMessageA(hwnd, WM_INPUTLANGCHANGEREQUEST, 0, currentLayout);
    }
}

fn main() {
    let Opt { ime } = Opt::from_args();

    match ime {
        Some(idx) => switch_ime_to(idx),
        None => {
            let currentLayout = get_ime_idx();
            println!("{}", currentLayout);
        }
    }
}
