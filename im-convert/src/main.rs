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
    HWND,
};

use structopt::StructOpt;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use dll_syringe::{Syringe, process::OwnedProcess};

// the message passed to our listener is one byte long.
#[derive(StructOpt)]
#[structopt(about = "A simple command that helps Chinese VSCodeVim users to switch IME")]
enum Cmd {
    Backup,
    Recover,
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

    // Send message.
    let mut written_bytes = 0;
    match cmd {
        Cmd::Backup => {
        },

        Cmd::Recover => {

        }
    }

}
