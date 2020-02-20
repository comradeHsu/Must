use crate::class_loader::class_loader::ClassLoader;
use crate::native::registry::Registry;
use crate::runtime_data_area::frame::Frame;
use crate::runtime_data_area::heap::class::Class;
use crate::utils::boxed;

pub fn init() {
    Registry::register(
        "sun/misc/URLClassPath",
        "getLookupCacheURLs",
        "(Ljava/lang/ClassLoader;)[Ljava/net/URL;",
        get_lookup_cache_urls,
    );
}

///private static native URL[] getLookupCacheURLs(ClassLoader var0);
///(Ljava/lang/ClassLoader;)[Ljava/net/URL;
pub fn get_lookup_cache_urls(frame: &mut Frame) {
    let vars = frame.local_vars().expect("vars is none");
    let java_loader = vars.get_ref(0);
    let class = frame.method().class();
    let class_loader = (*class).borrow().loader();
    let url_class = ClassLoader::load_class(class_loader.clone(), "java/net/URL");
    let array_class = (*url_class).borrow().array_class();
    let array = Class::new_array(&array_class, 0);
    frame
        .operand_stack()
        .expect("stack is none")
        .push_ref(Some(boxed(array)));
}
