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

use structopt::StructOpt;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

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
    let _thead_id = unsafe { GetWindowThreadProcessId(h_wnd, Some(&mut pid)) };
    let process = OwnedProcess::from_pid(pid)
        .expect("Get the process of the foreground window failed!");
    let syringe = Syringe::for_process(process);
    let injected_payload = syringe.find_or_inject("target/debug/im_conversion.dll")
        .expect("injection failed");

    let check_injected_process = unsafe {
        syringe.get_raw_procedure::<extern "system" fn() -> u32>(injected_payload, "check_injected_process")
            .unwrap().unwrap()
    };

    let remote_backup = unsafe {
        syringe.get_raw_procedure::<extern "system" fn() -> u32>(injected_payload, "backup")
            .unwrap().unwrap()
    };

    let remote_recover = unsafe {
        syringe.get_raw_procedure::<extern "system" fn()>(injected_payload, "recover")
            .unwrap().unwrap()
    };

    // before sending the message, check if the injected process is the same of the process call the binary
    let remote_pid = check_injected_process.call().unwrap();
    if remote_pid != pid {
        println!("process not the same! injected process: {remote_pid:x}, front window process: {pid:x}");
    }

    // Send message.
    match cmd {
        Cmd::Backup => {
            let res = remote_backup.call().unwrap();
            println!("remote backup result: {res:?}");
        },

        Cmd::Recover => {
            remote_recover.call().unwrap();
        }
    }

}
