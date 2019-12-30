use crate::cmd::Cmd;
use std::mem::take;
use std::process::exit;

mod cmd;

fn main() {
    let cmd = Cmd::parse_cmd();
    if cmd.version_flag {
        println!("java version \"1.8.0_152\"");
    } else if cmd.help_flag || cmd.class.as_str() == "" {
        Cmd::print_usage();
    }
    println!("Hello, world!");
}

fn start_jvm(cmd: &Cmd) {
    println!("classpath:{} class:{} args:{}\n", cmd.cp_option, cmd.class, cmd.args.get(0).unwrap());
}
