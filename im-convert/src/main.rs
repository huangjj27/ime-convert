//! Here is a reimplement of [`im-select`] in Rust, Which is in fact a client
//! to inject and send command to [`im-conversion-listener`], and only support
//! Windows (MacOS will be supportted in some day).
//!
//! ## Install
//! ```
//! cargo install im-convert
//! ```
//!
//! ## Manual
//! This CLI command provides two basic usages.
//! - To get current IME information, just run `im-convert backup`.
//! - To switch to preferred IME and conversion, run `im-convert recover`
//!
//! NOTES: these commands will only work on VSCodeVim config.
//!
//! [`im-select`]: https://github.com/daipeihust/im-select
//! [`im-conversion-listener`]: https://github.com/huangjj27/ime-convert/tree/main/im-conversion-listener

use windows_sys::Win32::Foundation::{
    HANDLE, HWND, INVALID_HANDLE_VALUE,
    BOOL, TRUE, FALSE,
    GENERIC_WRITE,
};
use windows_sys::Win32::Foundation::{
    GetLastError,
    CloseHandle,
};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::System::IO::OVERLAPPED;
use windows_sys::Win32::System::Mailslots::CreateMailslotA;
use windows_sys::Win32::System::Threading::WaitForSingleObject;
use windows_sys::Win32::Storage::FileSystem::{
    WriteFile,
    CreateFileA,
    OPEN_EXISTING,
    FILE_ATTRIBUTE_NORMAL,
};
use windows_sys::Win32::System::SystemServices::MAILSLOT_WAIT_FOREVER;


use structopt::StructOpt;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use dll_syringe::{Syringe, process::OwnedProcess};

// the message passed to our listener is one byte long.
const MSG_LENGTH: u32 = 1;
#[derive(StructOpt)]
#[structopt(about = "A simple command that helps Chinese VSCodeVim users to switch IME")]
enum Cmd {
    Backup,
    Recover,
}

#[non_exhaustive]
#[repr(u8)]
enum Msg {
    NeverUsed = 0b0000,
    Backup = 0b0001,
    Recover = 0b0010,
    Exit = 0b1000,
}

fn main() {
    let cmd = Cmd::from_args();

    // Inject or findout the dll.
    // Get the foreground window and its process id(`pid`),
    let h_wnd: HWND = unsafe { GetForegroundWindow() };
    let mut pid = 0;
    let _thead_id = unsafe { GetWindowThreadProcessId(h_wnd, &mut pid) };
    let process = OwnedProcess::from_pid(pid)
        .expect("Get the process of the foreground window failed!");
    // let syringe = Syringe::for_process(process);
    // syringe.find_or_inject("im_conversion_listener.dll")
    //     .expect("injection failed");

    let mailslot = format!("\\\\.\\mailslot\\im_conversion_listener_{pid:x}\0");
    let h_mailslot = unsafe {
        CreateFileA(
            mailslot.as_ptr(),
            GENERIC_WRITE,
            0,  // zero means that only current process can operate the file.
            std::ptr::null() as *const SECURITY_ATTRIBUTES,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            0,
        )
    };

    // Send message.
    let mut written_bytes = 0;
    match cmd {
        Cmd::Backup => {
            let backup = Msg::Backup;
            unsafe {
                WriteFile(
                    h_mailslot,
                    &backup as *const _ as _,
                    MSG_LENGTH,
                    &mut written_bytes,
                    std::ptr::null_mut() as *mut OVERLAPPED,
                );
            }
        },

        Cmd::Recover => {
            let recover = Msg::Recover;
            unsafe {
                WriteFile(
                    h_mailslot,
                    &recover as *const _ as _,
                    MSG_LENGTH,
                    &mut written_bytes,
                    std::ptr::null_mut() as *mut OVERLAPPED,
                );
            }
        }
    }

    unsafe {
        CloseHandle(h_mailslot);
    }
}
