use crate::utils::vecs::flat_map;

pub struct Cmd {
    pub help_flag: bool,
    pub version_flag: bool,
    pub verbose_class:bool,
    pub cp_option: Vec<String>,
    pub x_jre_option:String,
    pub class: String,
    pub args: Vec<String>
}

impl Cmd {
    pub fn new() -> Cmd {
        return Cmd {
            help_flag: false,
            version_flag: false,
            verbose_class: false,
            cp_option: vec![],
            x_jre_option: String::new(),
            class: String::new(),
            args: Vec::new()
        };
    }

    pub fn parse_cmd() -> Cmd {
        let mut args:Vec<String> = std::env::args().collect();
        let mut cmd:Cmd = Cmd::new();
        for mut index in 0..args.len() {
            let arg = args.get(index).unwrap();
            if arg.starts_with("-Xjre") {
                let mut array:Vec<&str> = arg.split(':').collect();
                cmd.x_jre_option = array.remove(1).to_string();
            }
            match arg.as_str() {
                "help" => cmd.help_flag = true,
                "?" => cmd.help_flag = true,
                "-version" => cmd.version_flag = true,
                "-verbose" => cmd.verbose_class = true,
                "-verbose:class" => cmd.verbose_class = true,
                "class" => cmd.class = arg.clone(),
                "-cp" | "-classpath"=> {
                    let null_ptr = "".to_string();
                    let mut param = args.get(index+1).unwrap_or_else(||{&null_ptr});
                    if param.starts_with("-") {
                        param = &null_ptr;
                    } else {
                        index += 1;
                    }
                    let array:Vec<&str> = param.split(';').collect();
                    let cps = flat_map::<String,&str>(array);
                    cmd.cp_option = cps;
                },
//                "Xjre" => cmd.x_jre_option = arg,
                _ => cmd.args.push(arg.clone())
            }
        }
        cmd.class = cmd.args.remove(1);
        return cmd;
    }

    pub fn print_usage() {
        println!("Usage: {} [-options] class [args...]\n", std::env::args().nth(0).unwrap());
    }
}
