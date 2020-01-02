use crate::cmd::Cmd;
use crate::class_path::class_path::{ClassPath, Entry};

mod cmd;
mod class_path;
mod class_file;
fn main() {
    let cmd = Cmd::parse_cmd();
    if cmd.version_flag {
        println!("java version \"1.8.0_152\"");
    } else if cmd.help_flag || cmd.class.as_str() == "" {
        Cmd::print_usage();
    } else {
        start_jvm(&cmd);
    }

    println!("Hello, world!");
}

fn start_jvm(cmd: &Cmd) {
    let cp = ClassPath::parse(&cmd.x_jre_option,&cmd.cp_option);
    println!("classpath:{} class:{} args:{}\n", cmd.cp_option, cmd.class, cmd.args.get(0).unwrap());
    let class_name = cmd.class.clone().replace('.',"/");
    let read_rs = cp.read_class(class_name.as_str());
    if read_rs.is_err() {
        println!("Could not find or load main class {}\n", cmd.class);
        return;
    }
    let (vecs, _point) = read_rs.unwrap();
    println!("class raw data :{:?}\n", vecs);
}

#[cfg(test)]
mod tests{
    use std::env;

    #[test]
    fn start_jvm() {
        for (key, value) in env::vars_os() {
            println!("{:?}: {:?}", key, value);
        }
    }
}
