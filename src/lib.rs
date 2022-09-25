//! Here define the functions that will inject to the foreground process, as an
//! injected dll.
//!
//! [`im-select`]: https://github.com/daipeihust/im-select

use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Globalization::HIMC;
use windows_sys::Win32::UI::Input::Ime::{
    ImmGetConversionStatus,
    ImmSetConversionStatus,
    ImmGetContext,
    ImmReleaseContext,
};

// extern "system" {
//     fn imm_get_conversion_status()
// }
