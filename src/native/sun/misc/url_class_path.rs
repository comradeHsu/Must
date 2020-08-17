use crate::class_loader::app_class_loader::ClassLoader;
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::oops::class::Class;
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
pub fn get_lookup_cache_urls(frame: &Frame) {
    let _java_loader = frame.get_ref(0);

    let url_class = Jvm::boot_class_loader()
        .find_or_create("java/net/URL")
        .unwrap();
    let array_class = (*url_class).borrow().array_class();
    let array = Class::new_array(&array_class, 0);
    frame.push_ref(Some(array));
}
