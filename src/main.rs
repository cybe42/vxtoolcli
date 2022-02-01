mod vxtcli;
mod cmd_manager;
//mod command_src/hello.rs

fn main() {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    vxtcli::start(String::from(VERSION.unwrap_or("unknown")));
}
