//! Here is a reimplement of [`im-select`] in Rust, Which provides different APIs
//! and only support Windows (MacOS will be supportted in some day).
//!
//! ## Install
//! ```
//! cargo install ime-conversion-vim
//! ```
//!
//! ## Manual
//! This CLI command provides two basic usages.
//! - To get current IME information, just run `ime-conversion-vim backup`. return whill be in `{conversion}` format
//! - To switch to preferred IME and conversion, run like `ime-conversion-vim recover {conversion}`
//!
//! NOTES: these commands will only work on VSCodeVim config.
//!
//! [`im-select`]: https://github.com/daipeihust/im-select

mod ime;

use ime::Ime;
use structopt::StructOpt;
use winapi::shared::minwindef::DWORD;

#[derive(StructOpt)]
#[structopt(about = "A simple command that helps Chinese VSCodeVim users to switch IME")]
enum Cmd {
    Backup,
    Recover {
        #[structopt()]
        conversion: DWORD,
    },
}

fn main() {
    let cmd = Cmd::from_args();
    let mut ime = Ime::new();

    match cmd {
        Cmd::Backup => println!("{}", ime.conversion()),
        Cmd::Recover { conversion } => ime.set_conversion(conversion),
    }
}
