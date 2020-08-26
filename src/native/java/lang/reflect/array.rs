use crate::native::registry::Registry;
use crate::runtime::frame::Frame;
use crate::oops::class::Class;

pub fn init() {
    Registry::register("java/lang/reflect/Array", "newArray", "(Ljava/lang/Class;I)Ljava/lang/Object;", new_array);
}

/// private static native Object newArray(Class<?> componentType, int length)
/// throws NegativeArraySizeException;
pub fn new_array(frame: &Frame) {
    let (component_type,length) = frame.local_vars_get(|vars|{
        let component_type = vars.get_ref(0);
        let length = vars.get_int(1);
        (component_type,length)
    });
    let component_class = component_type.unwrap().meta();
    let array_class = component_class.array_class();
    let array_object = Class::new_array(&array_class,length as usize);
    frame.push_ref(Some(array_object))
}