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

use windows_sys::Win32::Foundation::{
    HINSTANCE, HANDLE, HWND, INVALID_HANDLE_VALUE,
    BOOL
};
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;

use windows_sys::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
use windows_sys::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows_sys::Win32::System::Mailslots::CreateMailslotA;
use windows_sys::Win32::Storage::FileSystem::{
    ReadFile,
    WriteFile,
};
use windows_sys::Win32::System::IO::OVERLAPPED;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow,
    GetWindowThreadProcessId,
};

use std::os::windows::prelude::IntoRawHandle;
use std::sync::atomic::{ AtomicIsize, Ordering };

// the `BOOL` type from windows-sys defines zero as `FALSE` and non-zero as `TRUE`.
const FALSE: BOOL = 0i32;
// const TRUE: BOOL = 1i32;

// the message passed to our listener is one byte long.
const MSG_LENGTH: u32 = 1;
const EXIT: u8 = 0b1000;

/// A lazy initialized static to hold the listener thread.
static mut LISTENER: AtomicIsize = AtomicIsize::new(0);

/// A lazy initialized static for mailslot
static mut MAILSLOT: AtomicIsize = AtomicIsize::new(0);

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
            let h_mailslot: HANDLE = create_mailslot();
            unsafe {
                // SAFETY: MAILSLOT will be defined here first so
                // it will not be null.
                *MAILSLOT.get_mut() = h_mailslot;
            }

            let listener = std::thread::spawn(move || {
                let mut msg: u8 = 0;
                let mut read_bytes = 0;
                loop {
                    unsafe {
                        ReadFile(
                            h_mailslot,
                            &mut msg as *mut _ as _,
                            MSG_LENGTH,
                            &mut read_bytes,
                            0 as *mut OVERLAPPED,
                        );
                    }

                    match msg {
                        // notified to exit
                        0b1000 => {
                            break;
                        },

                        // to backup the im conve5sion status and switch to alpha mode
                        0b0001 => { },

                        // to recover the im conversion status.
                        0b0010 => { },

                        m @ _ => {
                            unreachable!("unexpected message {m} passed!");
                        }
                    }
                }
            });

            unsafe {
                // SAFETY: MAILSLOT will be defined here first so
                // it will not be null.
                *(LISTENER.get_mut()) = listener.into_raw_handle() as isize;
            }
        },

        // the `LISTENER` must be initialized when `DLL_PROCESS_ATTACH`.
        DLL_PROCESS_DETACH => {
            let mut written_bytes = 0;
            unsafe {
                WriteFile(
                    *MAILSLOT.get_mut(),
                    &EXIT as *const _ as _,
                    MSG_LENGTH,
                    &mut written_bytes,
                    0 as *mut OVERLAPPED,
                );

                let listener = LISTENER.swap(0, Ordering::AcqRel);

                CloseHandle(listener);
            }
        },

        _ => {
            unreachable!("This is a bug! unexpected fdwReason value: {fdwReason}");
        }
    }

    -1
}
