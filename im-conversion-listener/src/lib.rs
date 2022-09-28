//! Here is a reimplement of [`im-select`] in Rust, Which provides different APIs
//! and only support Windows now
//!
//! ## Manual
//! This CLI command provides two basic usages.
//! - To get current IME information, just run `ime-conversion-vim backup`. return whill be in `{conversion}` format
//! - To switch to preferred IME and conversion, run like `ime-conversion-vim recover {conversion}`
//!
//! NOTES: these commands will only work on VSCodeVim config.
//!
//! [`im-select`]: https://github.com/daipeihust/im-select

use windows_sys::Win32::Foundation::{HINSTANCE, BOOL};

use windows_sys::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
use windows_sys::Win32::System::LibraryLoader::DisableThreadLibraryCalls;

type DWORD = u32;
type LPVOID = *mut std::ffi::c_void;

// the `BOOL` type from windows-sys defines zero as `FALSE` and non-zero as `TRUE`.
const FALSE: BOOL = 0i32;

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: DWORD,
    _lpvReserved: LPVOID,
) -> BOOL {
    let disable_result: BOOL = unsafe { DisableThreadLibraryCalls(hinstDLL) };
    if disable_result == FALSE {
        panic!("DisableThreadLibraryCalls failed!");
    }

    match fdwReason {
        DLL_PROCESS_ATTACH => {
            // TODO: initialize a monitor thread that provides a mailslot.
            // the monitor thread should be initialized once.
        },
        DLL_PROCESS_DETACH => {
            // TODO: notify the monitor thread to exit.
        },
        _ => {
            unreachable!("This is a bug! unexpected fdwReason value: {fdwReason}");
        }
    }

    -1
}