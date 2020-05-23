//! Ime wrapper for VSCode window, Which should be fore-window when calling this binary.

use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::HWND;
use winapi::um::imm::ImmGetContext;
use winapi::um::imm::ImmGetConversionStatus;
use winapi::um::imm::ImmReleaseContext;
use winapi::um::imm::ImmSetConversionStatus;
use winapi::um::imm::HIMC;
use winapi::um::winuser::GetForegroundWindow;

// use std::raw::CString;

/// Ime wrapper for VSCode window, Which should be fore window when calling this binary.
pub struct Ime {
    win: WindowHandle,
    handle: ImeHandle,
}

type WindowHandle = HWND;
type ImeHandle = HIMC;
type ConversionStatus = DWORD;

impl Ime {
    pub fn new() -> Self {
        // VSCode window should always be the foreground window
        let hwnd = unsafe { GetForegroundWindow() };

        Self::for_window(hwnd)
    }

    pub fn conversion(&self) -> ConversionStatus {
        let (mut c, mut s) = (0, 0);

        match unsafe { ImmGetConversionStatus(self.handle, &mut c, &mut s) } {
            TRUE => c,
            FALSE => panic!("Converting failed!"),
            _ => unreachable!("Should not get value other than TRUE or FALSE"),
        }
    }

    // when we set conversion, we use self::handle to modify the Window's conversion.
    pub fn set_conversion(&mut self, cs: ConversionStatus) {
        match unsafe { ImmSetConversionStatus(self.handle, cs, 0) } {
            TRUE => return,
            FALSE => panic!("Recovering failed!"),
            _ => unreachable!("Should not get value other than TRUE or FALSE"),
        }
    }

    fn for_window(win_handle: HWND) -> Self {
        // the handle will be allways a null pointer if we don't own that window.
        let himc = unsafe { ImmGetContext(win_handle) };
        assert!(!himc.is_null(), "We should get imm for that window!");

        Self {
            win: win_handle,
            handle: himc,
        }
    }
}

impl Drop for Ime {
    fn drop(&mut self) {
        unsafe {
            match ImmReleaseContext(self.win, self.handle) {
                TRUE => println!("released!"),
                FALSE => println!("Error while releasing!"),
                _ => unreachable!("Should not get value other than TRUE or FALSE"),
            }
        }
    }
}
