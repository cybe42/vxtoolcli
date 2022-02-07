//use crate::cmd_manager;

#[cfg(target_os = "linux")]
use termion::{clear, cursor};


use std::io;
use std::io::Write;

pub fn command(_args: Vec<&str>) -> bool {
    print!("{}", clear::All);
    print!("{}", cursor::Goto(1,1));
    io::stdout().flush().unwrap();
    return false;
}