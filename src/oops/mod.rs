pub mod access_flags;
pub mod array_object;
pub mod class;
pub mod class_member;
pub(crate) mod class_name_helper;
pub mod class_ref;
pub mod constant_pool;
mod exception_table;
pub mod field;
pub mod field_ref;
pub mod interface_method_ref;
pub mod member_ref;
pub mod method;
pub mod method_descriptor;
pub mod method_ref;
pub mod object;
pub mod slots;
pub mod string_pool;
pub mod sym_ref;

#[cfg(test)]
mod test {
    use std::cell::UnsafeCell;

    #[test]
    fn test_unsafe_cell() {
        unsafe {
            let data = UnsafeCell::new(10);
            let s1 = &mut *data.get();
            let s2 = &mut *data.get();
            *s1 = 5;
            println!("data:{}", *data.get());
            *s2 = 15;
            println!("data:{}", *data.get());
        }
    }
}
