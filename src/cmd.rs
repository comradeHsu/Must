pub struct Cmd {
    pub help_flag: bool,
    pub version_flag: bool,
    pub cp_option: String,
    pub class: String,
    pub args: Vec<String>
}

impl Cmd {
    pub const fn new() -> Cmd {
        return Cmd {
            help_flag: false,
            version_flag: false,
            cp_option: String::new(),
            class: String::new(),
            args: Vec::new()
        };
    }

    pub fn parse_cmd() -> Cmd {
        let mut args = std::env::args();
        let mut cmd:Cmd = Cmd::new();
        for arg in args {
            match arg.as_str() {
                "help" => cmd.help_flag = true,
                "?" => cmd.help_flag = true,
                "version" => cmd.version_flag = true,
                "class" => cmd.class = arg,
                "cp" => cmd.cp_option = arg,
                "classPath" => cmd.cp_option = arg,
                _ => cmd.args.push(arg)
            }
        }
        return cmd;
    }

    pub fn print_usage() {
        println!("Usage: {} [-options] class [args...]\n", std::env::args().nth(0).unwrap());
    }
}
