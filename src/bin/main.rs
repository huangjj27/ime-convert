//! Here is a reimplement of [`im-select`] in Rust, Which provides different APIs
//! and only support Windows (MacOS will be supportted in some day).
//!
//! ## Install
//! ```
//! cargo install ime-conversion-vim
//! ```
//!
//! ## Manual
//! This CLI command provides two basic usages.
//! - To get current IME information, just run `ime-conversion-vim backup`. return whill be in `{conversion}` format
//! - To switch to preferred IME and conversion, run like `ime-conversion-vim recover {conversion}`
//!
//! NOTES: these commands will only work on VSCodeVim config.
//!
//! [`im-select`]: https://github.com/daipeihust/im-select


use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A simple command that helps Chinese VSCodeVim users to switch IME")]
enum Cmd {
    Backup,
    Recover {
        #[structopt()]
        conversion: u32,
    },
}

fn main() {
    let cmd = Cmd::from_args();
    let mut ime = Ime::new();

    match cmd {
        Cmd::Backup => println!("{}", ime.conversion()),
        Cmd::Recover { conversion } => ime.set_conversion(conversion),
    }
}

use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Globalization::HIMC;
use windows_sys::Win32::UI::Input::Ime::{
    ImmGetConversionStatus,
    ImmSetConversionStatus,
    ImmGetContext,
    ImmReleaseContext,
};
use windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

// use std::raw::CString;

/// Ime wrapper for VSCode window, Which should be fore window when calling this binary.
pub struct Ime {
    win: WindowHandle,
    handle: ImeHandle,
}

type WindowHandle = HWND;
type ImeHandle = HIMC;

impl Ime {
    pub fn new() -> Self {
        // VSCode window should always be the foreground window
        let hwnd = unsafe { GetForegroundWindow() };

        Self::for_window(hwnd)
    }

    pub fn conversion(&self) -> u32 {
        let (mut conversion, mut sentence) = (0, 0);

        match unsafe {
            ImmGetConversionStatus(self.handle,
                &mut conversion,
                &mut sentence,
            )
        } {
            0 => panic!("Converting failed!"),
            _ => conversion,
        }
    }

    // when we set conversion, we use self::handle to modify the Window's conversion.
    pub fn set_conversion(&mut self, cs: u32) {
        match unsafe { ImmSetConversionStatus(self.handle, cs, 0) } {
            0 => panic!("Recovering failed!"),
            _ => return,
        }
    }

    fn for_window(win_handle: HWND) -> Self {
        // the handle will be always a null pointer if we don't own that window.
        let himc: HIMC = unsafe { ImmGetContext(win_handle) };
        assert!(himc != 0, "We should get imm for that window!");

        Self {
            win: win_handle,
            handle: himc,
        }
    }
}

impl Drop for Ime {
    fn drop(&mut self) {
        match unsafe { ImmReleaseContext(self.win, self.handle) } {
            0 => println!("Error while releasing!"),
            _ => println!("released!"),
        }
    }
}
