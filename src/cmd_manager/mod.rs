use std::collections::HashMap;
mod help;

static COMMANDS: &'static [&'static str] = &[
    "help"
];

static CMD_SOURCE: &'static [fn(Vec<&str>)->bool] = &[
    help::command
];

static CMD_HELP: &'static [&'static str] = &[
    "help [<command>] - Displays all commands and their usages."
];


pub fn return_commands() ->HashMap<String, fn(Vec<&str>)->bool> {
    let mut cmds: HashMap<String, fn(Vec<&str>)->bool> =  HashMap::new();
    for i in 0..CMD_SOURCE.len() {
        cmds.insert(String::from(COMMANDS[i]), CMD_SOURCE[i]);
    }
    return cmds;
}

pub fn return_help() ->HashMap<String, String> {
    let mut help_cmds: HashMap<String, String> = HashMap::new();
    for i in 0..CMD_HELP.len() {
        help_cmds.insert(String::from(COMMANDS[i]), String::from(CMD_HELP[i]));
    }
    return help_cmds; 
}