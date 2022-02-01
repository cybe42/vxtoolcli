use std::io;
use std::io::Write;
use termion::{color};
use std::collections::HashMap;

use crate::cmd_manager;

fn input_loop() {
    
    print!("{}   Enter command> ", color::Fg(color::Green));
    io::stdout().flush().unwrap();
    let mut raw_command = String::new();
    io::stdin().read_line(&mut raw_command)
        .ok()
        .expect(format!("{}I could not read a line from stdin.", color::Fg(color::Red)).as_str());

    let command: Vec<&str> = raw_command.trim().split(" ").collect();
    let mut cmd_head: String = String::new();
    if command.len() > 0 {
        cmd_head = String::from(command[0]);
    } else {
        cmd_head = String::from("");
    }
    let cmds: HashMap<String, fn(Vec<&str>)> = cmd_manager::return_commands();
    if !cmds.contains_key(&cmd_head) {
        println!("      Could not find a command called \"{}\"", cmd_head);
    } else {
        let func = cmds.get(&cmd_head).unwrap();
        //func();
        //func();
        func(command);
    }
}

pub fn start(version: String) {
    let mut banner = String::from(r###"___    ______  __________              ______        
__ |  / /__  |/ /___  __/______ ______ ___  /________
__ | / / __    / __  /   _  __ \_  __ \__  / __  ___/
__ |/ /  _    |  _  /    / /_/ // /_/ /_  /  _(__  ) 
_____/   /_/|_|  /_/     \____/ \____/ /_/   /____/  
                                                     
"###);
    banner = banner + "VXTools v"+ &version;
    println!("{}{}", color::Fg(color::Red), banner);
    loop {
        input_loop();
    }
}