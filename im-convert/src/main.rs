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
    HINSTANCE, HANDLE, HWND, INVALID_HANDLE_VALUE,
    BOOL
};
use windows_sys::Win32::Foundation::{
    GetLastError,
    CloseHandle,
};
use windows_sys::Win32::System::Threading::WaitForSingleObject;
use windows_sys::Win32::Storage::FileSystem::{
    WriteFile,
    CreateFileA,
    OPEN_EXISTING,
};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A simple command that helps Chinese VSCodeVim users to switch IME")]
enum Cmd {
    Backup,
    Recover,
}

fn get_mailslot() -> HANDLE {
    // Get the foreground window and its process id(`pid`),
    let h_wnd: HWND = unsafe { GetForegroundWindow() };
    let mut pid = 0;
    let _thead_id = unsafe { GetWindowThreadProcessId(h_wnd, &mut pid) };

    // create a mailslot based on the `pid`
    let mailslot = format!("\\\\.\\mailsot\\im_conversion_listener_{pid:x}");
}


fn main() {
    let cmd = Cmd::from_args();

    // Inject or findout the dll.

    // Send message.
    match cmd {
        Cmd::Backup => println!("{}", ime.conversion()),
        Cmd::Recover { conversion } => ime.set_conversion(conversion),
    }
}
