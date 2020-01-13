mod cmd;
mod class_path;
mod class_file;
mod runtime_data_area;
mod utils;
mod instructions;
mod interpreter;

use crate::cmd::Cmd;
use crate::class_path::class_path::{ClassPath, Entry};
use crate::class_file::class_file::ClassFile;
use crate::class_file::member_info::{display_16, MemberInfo};
use crate::interpreter::interpret;

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
    let class_file = load_class(&class_name,&cp);
    class_file.display();
    let main = get_main_method(&class_file);
    if main.is_some() {
        interpret(main.unwrap());
    }
//    let read_rs = cp.read_class(class_name.as_str());
//    if read_rs.is_err() {
//        println!("Could not find or load main class {}\n", cmd.class);
//        return;
//    }
//    let (vecs, _point) = read_rs.unwrap();
//    println!("class raw data :{}\n", class_file.to_string());
}

fn load_class(class_name:&String,cp:&ClassPath) -> ClassFile {
    let (class_data,_entry) = cp.read_class(class_name.as_str()).unwrap();
    let class_file = ClassFile::parse(class_data);
    return class_file;
}

fn get_main_method(class_file:&ClassFile) -> Option<&MemberInfo> {
    for method in class_file.methods() {
        if  method.name() == "main" && method.descriptor() ==  "([Ljava/lang/String;)V" {
            return Some(method);
        }
    }
    return None;
}

#[cfg(test)]
mod tests{
    use std::env;
    use std::time::SystemTime;
    use crate::class_path::class_path::ClassPath;
    use crate::{load_class, get_main_method};
    use crate::interpreter::interpret;
    use crate::cmd::Cmd;

    #[test]
    fn start_jvm() {
        let cmd = Cmd{
            help_flag: false,
            version_flag: false,
            cp_option: "D:/workspace/rust-jvm".to_string(),
            x_jre_option: "".to_string(),
            class: "java.GuassTest".to_string(),
            args: vec![]
        };
        let cp = ClassPath::parse(&cmd.x_jre_option,&cmd.cp_option);
        let class_name = cmd.class.clone().replace('.',"/");
        let class_file = load_class(&class_name,&cp);
        class_file.display();
        let main = get_main_method(&class_file);
        if main.is_some() {
            interpret(main.unwrap());
        }
    }

    #[test]
    fn start_jvm_main() {
        let cmd = Cmd{
            help_flag: false,
            version_flag: false,
            cp_option: "D:/test".to_string(),
            x_jre_option: "".to_string(),
            class: "com.compile.Main".to_string(),
            args: vec![]
        };
        let cp = ClassPath::parse(&cmd.x_jre_option,&cmd.cp_option);
        let class_name = cmd.class.clone().replace('.',"/");
        let class_file = load_class(&class_name,&cp);
        class_file.display();
        let main = get_main_method(&class_file);
        if main.is_some() {
            interpret(main.unwrap());
        }
    }

    #[test]
    fn test_time() {
        #[inline]
        fn dfs(row:i32, shu:i32, pie:i32, na:i32,mut cnt:i32) {
            let n = 15;
            let mut ave = ((1 << n) - 1) & (-((shu | pie | na)+1)) ;
            while ave != 0  {
                let p = ave & -ave ;
                ave ^= p ;
                if row == n {
                    cnt+=1;
                } else {
                    dfs(row + 1, shu | p, (pie | p) >> 1, (na | p) << 1,cnt);
                }
            }
        }
        let start = SystemTime::now();
        dfs(1,0,0,0,0);
        let end = SystemTime::now();
        println!("{:?}: {:?}", start, end);
    }
}
