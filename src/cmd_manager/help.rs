use crate::cmd_manager;
use std::collections::HashMap;

fn print_command_list() {
    let commandlist: HashMap<String, fn(Vec<&str>)->bool> = cmd_manager::return_commands();
    let cmdhelplist: HashMap<String, String> = cmd_manager::return_help();
    for i in commandlist.iter() {
        match cmdhelplist.contains_key(i.0) {
            true => println!("      {command_name}: {usage}",
                        command_name=i.0,
                        usage=cmdhelplist.get(i.0).unwrap()
            ),
                
            false => println!("      {command_name}: {usage}",
                        command_name=i.0,
                        usage="No help/usage information found about command."
            ),
        }
    }
}

pub fn command(_args: Vec<&str>) -> bool {
    let cmdhelplist: HashMap<String, String> = cmd_manager::return_help();
    if _args.len() > 1 {
        let spec_cmd = String::from(_args[2]).to_lowercase();
        if cmdhelplist.contains_key(&spec_cmd) {
            println!("      {command_name}: {usage}",
                command_name=spec_cmd,
                usage=cmdhelplist.get(&spec_cmd).unwrap());
        } else {
            print_command_list();
        }
    } else {
        print_command_list();
    }
    return false;
}