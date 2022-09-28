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
    HINSTANCE, HANDLE, INVALID_HANDLE_VALUE,
    BOOL
};
use windows_sys::core::PCSTR;

use windows_sys::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
use windows_sys::Win32::System::LibraryLoader::DisableThreadLibraryCalls;

use std::thread::JoinHandle;

// the `BOOL` type from windows-sys defines zero as `FALSE` and non-zero as `TRUE`.
const FALSE: BOOL = 0i32;

/// A lazy initialized static to hold the listener.
static mut LISTENER: Lazy<Listener> = Lazy::new(Listener::spawn);

/// A listener spawns a worker thread that holds a mailslot combined with the
/// loading process which receives command from an outer executable. The worker
/// thread
struct Listener {
    worker: Option<JoinHandle<BOOL>>,
}

impl Listener {
    fn spawn() -> Self {
        todo!("Get the foreground window and its process id(`pid`),
            and create a mailslot based on the `pid`, and spawn
            a new worker thread");
    }

    fn exit(&mut self) {
        todo!("notify the worker thread to close the mailslot,
            and wait the thread to exit.");
    }
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
            // SAFETY: the initialization is only happend once when
            // `DLL_PROCESS_ATTACH`.
            unsafe {
                Lazy::force(&LISTENER);
            }
        },

        // the `LISTENER` must be initialized when `DLL_PROCESS_ATTACH`.
        DLL_PROCESS_DETACH => {
            // SAFETY: the only mutable access of `LISTENER` is happenned
            // when it's ready to call `Listener.exit`.
            let listener: &mut Listener = unsafe {
                Lazy::get_mut(&mut LISTENER)
                    .expect("the `LISTENER` must be initialized!")
            };
            listener.exit();
        },
        _ => {
            unreachable!("This is a bug! unexpected fdwReason value: {fdwReason}");
        }
    }

    -1
}