//use crate::cmd_manager;
use termion::{clear, cursor};
use std::io;
use std::io::Write;

pub fn command(_args: Vec<&str>) -> bool {
    print!("{}", clear::All);
    print!("{}", cursor::Goto(1,1));
    io::stdout().flush().unwrap();
    return false;
}