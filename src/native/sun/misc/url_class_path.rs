
use crate::jvm::Jvm;
use crate::native::registry::Registry;
use crate::oops::class::Class;
use crate::runtime::frame::Frame;


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
    let array_class = url_class.array_class();
    let array = Class::new_array(&array_class, 0);
    frame.push_ref(Some(array));
}
