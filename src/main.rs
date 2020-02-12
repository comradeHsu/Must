use crate::cmd::Cmd;
use crate::jvm::Jvm;

mod class_file;
mod class_path;
mod cmd;
mod instructions;
mod interpreter;
mod jvm;
mod native;
mod runtime_data_area;
mod utils;

fn main() {
    let cmd = Cmd::parse_cmd();
    if cmd.version_flag {
        println!("java version \"1.8.0_152\"");
    } else if cmd.help_flag || cmd.class.as_str() == "" {
        Cmd::print_usage();
    } else {
        Jvm::new(cmd).start();
    }
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::cmd::Cmd;
    use crate::jvm::Jvm;
    use std::time::SystemTime;

    #[test]
    fn start_jvm() {
        let cmd = Cmd {
            help_flag: false,
            version_flag: false,
            verbose_class: false,
            cp_option: "D:/workspace/rust-jvm".to_string(),
            x_jre_option: "".to_string(),
            class: "testJava.BubbleSortTest".to_string(),
            args: vec![],
        };
        let vec = "ha哈哈";
        let s: Vec<u16> = vec.encode_utf16().collect();
        println!("vec {:?}", s);
        Jvm::new(cmd).start();
    }

    #[test]
    fn start_jvm_main() {
        let cmd = Cmd {
            help_flag: false,
            version_flag: false,
            verbose_class: false,
            cp_option: "D:/test".to_string(),
            x_jre_option: "".to_string(),
            class: "com.compile.Main".to_string(),
            args: vec![],
        };
        Jvm::new(cmd).start();
    }

    #[test]
    fn test_time() {
        #[inline]
        fn dfs(row: i32, shu: i32, pie: i32, na: i32, mut cnt: i32) {
            let n = 15;
            let mut ave = ((1 << n) - 1) & (-((shu | pie | na) + 1));
            while ave != 0 {
                let p = ave & -ave;
                ave ^= p;
                if row == n {
                    cnt += 1;
                } else {
                    dfs(row + 1, shu | p, (pie | p) >> 1, (na | p) << 1, cnt);
                }
            }
        }
        let start = SystemTime::now();
        dfs(1, 0, 0, 0, 0);
        let end = SystemTime::now();
        let ptr = &end as *const SystemTime;
        let i = 99;
        let c = 99;
        println!("{:?}: {:?},{}", start, end, ptr as usize);
        let p = &i as *const i32;
        let add = p as usize;
        let t = add as *const i32;
        println!("{}: {}", add, unsafe { *t });
        let p = &c as *const i32;
        let add = p as usize;
        let t = add as *const i32;
        println!("{}: {}", add, unsafe { *t });
    }
}
