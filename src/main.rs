use crate::cmd::Cmd;
use std::mem::take;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::class_path::class_path::read_to_vec;

mod cmd;
mod class_path;
fn main() {
    let cmd = Cmd::parse_cmd();
    if cmd.version_flag {
        println!("java version \"1.8.0_152\"");
    } else if cmd.help_flag || cmd.class.as_str() == "" {
        Cmd::print_usage();
    }
    let path = Path::new("C:\\Users\\xuhui\\Desktop\\css.zip");
    let zip_file = File::open(path).unwrap();
    let mut reader = std::io::Cursor::new(read_to_vec(zip_file));
    let mut zip = zip::ZipArchive::new(reader).unwrap();
    let mut bytes:Vec<u8> = Vec::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        println!("Hello, {}!",file.name());
    }

    println!("Hello, world!");
}

fn start_jvm(cmd: &Cmd) {
    println!("classpath:{} class:{} args:{}\n", cmd.cp_option, cmd.class, cmd.args.get(0).unwrap());
}
