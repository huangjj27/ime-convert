//! Here is a reimplement of [`im-select`] in Rust, Which provides different APIs
//! and only support Windows (MacOS will be supportted in some day).
//!
//! ## Install
//! ```
//! cargo install im-select-rs
//! ```
//!
//! ## Manual
//! This CLI command provides two basic usages.
//! - To get current IME information, just run `im-select get`. return whill be in `{IME}.{conversion}` format
//! - To switch to preferred IME and conversion, run like `im-select set {IME}.{conversion}`
//!
//! NOTES: these commands will only work on VSCodeVim config.
//!
//! [`im-select`]: https://github.com/daipeihust/im-select

use structopt::StructOpt;

mod ime;

#[derive(StructOpt)]
#[structopt(about="A simple command that helps Chinese VSCodeVim users to switch IME")]
enum Cmd {
    Get,
    Set {
        #[structopt()]
        ImeDotConversion: String,
    },
}

fn main() {
    let cmd = Cmd::from_args();
    // let mut ime = Ime::new();

    match cmd {
        Cmd::Get => {
            // ime.name_and_conversion(),

            println!("Get!");
        }
        Cmd::Set { ImeDotConversion } => {
            // parse iac string
            // make it a setter chain?
            // ime.set().set_conversion();

            println!("Set!");
        }
    }

}
