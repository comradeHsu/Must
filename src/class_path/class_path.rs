use std::fs::File;
use std::io::Read;

pub trait Entry where Self:ToString{

    fn read_class(self,class_name:String) -> (Vec<u8>,Box<dyn Entry>);

}

pub fn read_to_vec(file:File) -> Vec<u8> {
    let mut zip_bytes = Vec::new();
    file.bytes().for_each(|x| zip_bytes.push(x.unwrap()));
    return zip_bytes;
}