use std::collections::HashMap;

mod help;

pub fn return_commands() ->HashMap<String, fn(Vec<&str>)> {
    let mut cmds: HashMap<String, fn(Vec<&str>)> = HashMap::new();
    cmds.insert(String::from("help"), help::command);
    return cmds;
}