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

use once_cell::sync::Lazy;
use windows_sys::Win32::Foundation::{
    HINSTANCE, HANDLE, HWND, INVALID_HANDLE_VALUE,
    BOOL
};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;

use windows_sys::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
use windows_sys::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows_sys::Win32::System::Mailslots::CreateMailslotA;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow,
    GetWindowThreadProcessId,
};

use std::sync::atomic::AtomicIsize;

// the `BOOL` type from windows-sys defines zero as `FALSE` and non-zero as `TRUE`.
const FALSE: BOOL = 0i32;
const TRUE: BOOL = 1i32;

/// A lazy initialized static to hold the listener thread.
static mut LISTENER: AtomicIsize = AtomicIsize::new(0);

/// A lazy initialized static for mailslot identifier.
static MAILSLOT: Lazy<HANDLE> = Lazy::new(create_mailslot);

fn create_mailslot() -> HANDLE {
    // Get the foreground window and its process id(`pid`),
    let h_wnd: HWND = unsafe { GetForegroundWindow() };
    let mut pid = 0;
    let _thead_id = unsafe { GetWindowThreadProcessId(h_wnd, &mut pid) };

    // create a mailslot based on the `pid`
    let mailslot_name = format!("\\\\.\\mailsot\\im_conversion_listener_{pid:x}");

    let h_mailslot: HANDLE = unsafe {
        CreateMailslotA(
            mailslot_name.as_ptr(),
            1,
            0,
            0 as *const SECURITY_ATTRIBUTES,
        )
    };

    if h_mailslot == INVALID_HANDLE_VALUE {
        panic!("mailslot for failed to create!");
    }

    h_mailslot
}

/// Spawn a thread to keep listening the MAILSLOT
///
/// ## SAFETY
/// We assume this function is always called before `exit`
fn spawn() {
    let handle = std::thread::spawn(|| {

    });
}

/// Exit the listener thread
///
/// ## SAFETY
/// We assume this function is always called after we have initially `spawn`
fn exit() {
    todo!("notify the worker thread to close the mailslot,
        and wait the thread to exit.");
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: u32,
    _lpvReserved: *mut std::ffi::c_void,
) -> BOOL {
    let disable_result: BOOL = unsafe {
        // This dll doesn't care about what happens to the threads
        // created by the process.
        DisableThreadLibraryCalls(hinstDLL)
    };
    if disable_result == FALSE {
        panic!("DisableThreadLibraryCalls failed!");
    }

    match fdwReason {
        // Initialize a listener thread that provides a mailslot.
        // the listener thread should be initialized once.
        DLL_PROCESS_ATTACH => {
            spawn();
        },

        // the `LISTENER` must be initialized when `DLL_PROCESS_ATTACH`.
        DLL_PROCESS_DETACH => {
            exit();
        },
        _ => {
            unreachable!("This is a bug! unexpected fdwReason value: {fdwReason}");
        }
    }

    -1
}
