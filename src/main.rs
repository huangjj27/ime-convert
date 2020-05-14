#![allow(non_snake_case)]

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
    let currentLayout = unsafe {
        let hwnd = GetForegroundWindow();
        let threadID = GetWindowThreadProcessId(hwnd, std::ptr::null_mut::<DWORD>());
        GetKeyboardLayout(threadID)
    };

    LOWORD(currentLayout as DWORD)
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
