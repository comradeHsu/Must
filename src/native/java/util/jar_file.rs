use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::array_object::ArrayObject;
use crate::runtime_data_area::heap::object::DataType;
use crate::runtime_data_area::heap::string_pool::StringPool;
use crate::utils::boxed;

pub fn init() {
    Registry::register(
        "java/util/jar/JarFile",
        "getMetaInfEntryNames",
        "()[Ljava/lang/String;",
        get_meta_inf_entry_names,
    );
}

///private native String[] getMetaInfEntryNames();
/// ()[Ljava/lang/String;
pub fn get_meta_inf_entry_names(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let this = vars.get_this().unwrap();
    let address = (*this).borrow().get_long_var("jzfile", "J") as usize;
    let zip_file = crate::native::java::util::zip_file::zip_file_cache::get_mut(address)
        .expect("the file is not open");
    let mut data = Vec::new();
    if zip_file.file.by_name("META-INF/").is_some() {
        data.push(Some(StringPool::java_string("META-INF/".to_string())));
    }
    if zip_file.file.by_name("META-INF/MANIFEST.MF").is_some() {
        data.push(Some(StringPool::java_string(
            "META-INF/MANIFEST.MF".to_string(),
        )));
    }
    let boot = Jvm::boot_class_loader();
    let object = ArrayObject::from_data(
        boot.find_or_create("java/lang/String").unwrap(),
        DataType::References(data),
    );
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(Some(boxed(object)));
}
