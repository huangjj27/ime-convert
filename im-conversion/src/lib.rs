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

use windows::Win32::Foundation::{
    HINSTANCE, HWND,
    BOOL, TRUE, FALSE,
};
use windows::Win32::Globalization::HIMC;
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow,
    GetWindowThreadProcessId,
};
use windows::Win32::UI::Input::Ime::{
    ImmGetContext, ImmReleaseContext,
    ImmGetConversionStatus, ImmSetConversionStatus, IME_CONVERSION_MODE, IME_SMODE_AUTOMATIC,
};
use windows::Win32::UI::Input::Ime::IME_CMODE_ALPHANUMERIC;

use std::collections::HashMap;
use std::sync::{ OnceLock, Mutex };

// the key `isize` is the inner of HWND
static CONVERSIONS: OnceLock<Mutex<HashMap<isize, IME_CONVERSION_MODE>>> = OnceLock::new();

#[no_mangle]
#[allow(unused)]
extern "system" fn check_injected_process() -> u32 {
    // Get the foreground window and its process id(`pid`),
    let h_wnd: HWND = unsafe { GetForegroundWindow() };
    let mut pid = 0;
    let _thead_id = unsafe { GetWindowThreadProcessId(h_wnd, Some(&mut pid)) };

    pid
}

/// Backup the IME conversion and set the convertion to `IME_CMODE_ALPHANUMERIC`
#[no_mangle]
#[allow(unused)]
extern "system" fn backup() {
    // the ForegroundWindow of a process may change, so we have to
    // get the window first each time we are about to backup/recover
    // the IME conversion.
    let hwnd: HWND = unsafe { GetForegroundWindow() };
    let himc: HIMC = unsafe { ImmGetContext(hwnd) };

    let mut conversion = IME_CMODE_ALPHANUMERIC;

    let get_res: BOOL = unsafe {
        ImmGetConversionStatus(
            himc,
            Some(&mut conversion),
            None,
        )
    };

    if get_res == FALSE {
        todo!("failure for ImmGetConversionStatus need to be handled!");
    }

    let mut conversions = CONVERSIONS
        .get()
        .expect("Init CONVERSIONS failed!")
        .lock()
        .expect("Get conversions failed!");

    // hack: `HWND` doesn't satisfy `HWND: hash`, but the isize value behind it does.
    (*conversions)
        .entry(hwnd.0)
        .or_insert(conversion);


    let set_res: BOOL = unsafe {
        ImmSetConversionStatus(
            himc,
            IME_CMODE_ALPHANUMERIC,
            IME_SMODE_AUTOMATIC,
        )
    };

    if set_res == FALSE {
        todo!("failure for ImmSetConversionStatus need to be handled!");
    }

    let release_res: BOOL = unsafe {
        ImmReleaseContext(hwnd, himc)
    };

    if release_res == FALSE {
        todo!("failure for ImmReleaseContext need to be handled!");
    }
}

/// recover the IME conversion from the recorded CONVERSIONS map.
#[no_mangle]
#[allow(unused)]
extern "system" fn recover() {
    let hwnd: HWND = unsafe { GetForegroundWindow() };
    let himc: HIMC = unsafe { ImmGetContext(hwnd) };

    let conversions = CONVERSIONS
        .get()
        .expect("Init CONVERSIONS failed!")
        .lock()
        .expect("Get conversions failed!");

    let conversion = (*conversions).get(&hwnd.0).unwrap();

    let set_res: BOOL = unsafe {
        ImmSetConversionStatus(
            himc,
            *conversion,
            IME_SMODE_AUTOMATIC,
        )
    };

    if set_res == FALSE {
        todo!("failure for ImmSetConversionStatus need to be handled!");
    }

    let release_res: BOOL = unsafe {
        ImmReleaseContext(hwnd, himc)
    };

    if release_res == FALSE {
        todo!("failure for ImmReleaseContext need to be handled!");
    }
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: u32,
    _lpvReserved: *mut std::ffi::c_void,
) -> BOOL {
    let disable_result = unsafe {
        // This dll doesn't care about what happens to the threads
        // created by the process.
        DisableThreadLibraryCalls(hinstDLL)
    };
    if let Err(_) = disable_result {
        // todo!("DisableThreadLibraryCalls failed!");
        return FALSE;
    }

    match fdwReason {
        DLL_PROCESS_ATTACH => {
            let _conversions = CONVERSIONS.get_or_init(|| {
                Mutex::new(HashMap::new())
            });
        },

        DLL_PROCESS_DETACH => (),

        _ => {
            // TODO: any other reason to call this dll should never happen.
            return FALSE;
        }
    }

    return TRUE;
}
