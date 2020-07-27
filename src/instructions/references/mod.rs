use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::constant_pool::Constant::MethodReference;

pub mod anew_array;
pub mod array_length;
pub mod athrow;
pub mod check_cast;
pub mod get_field;
pub mod get_static;
pub mod instance_of;
pub mod invoke_interface;
pub mod invoke_special;
pub mod invoke_static;
pub mod invoke_virtual;
pub mod monitor;
pub mod multi_anew_array;
pub mod new;
pub mod new_array;
pub mod put_field;
pub mod put_static;

trait ResolveFieldRef {
    fn resolve_field_ref(&self,class:Rc<RefCell<Class>>) -> Rc<RefCell<Field>> {
        let mut class_mut = (*class).borrow_mut();
        let cp = class_mut.mut_constant_pool();
        return cp.resolve_field_ref(self.get_index_in_constant_pool()).unwrap();
    }

    fn get_index_in_constant_pool(&self) -> usize;
}

trait ResolveClassRef {
    fn resolve_class_ref(&self,class:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        let mut class_mut = (*class).borrow_mut();
        let cp = class_mut.mut_constant_pool();
        return cp.resolve_class_ref(self.get_index_in_constant_pool());
    }

    fn get_index_in_constant_pool(&self) -> usize;
}

trait ResolveMethodRef {
    fn resolved_method_ref(&self,class:Rc<RefCell<Class>>) -> Rc<Method> {
        let mut class_mut = (*class).borrow_mut();
        let cp = class_mut.mut_constant_pool();
        return cp.resolve_method_ref(self.get_index_in_constant_pool()).unwrap();
    }

    fn resolved_method_ref_tuple(&self,class:Rc<RefCell<Class>>) -> (Rc<RefCell<Class>>,Rc<Method>) {
        let mut class_mut = (*class).borrow_mut();
        println!("1111");
        let cp = class_mut.mut_constant_pool();
        let constant = cp.get_constant(self.get_index_in_constant_pool());
        let method_ref = match constant {
            MethodReference(c) => c,
            _ => panic!("Unknown constant type"),
        };
        let resolved_class = method_ref.resolved_class();
        println!("2222");
        let resolved_method = method_ref.resolved_method().unwrap();
        return (resolved_class,resolved_method);
    }

    fn get_index_in_constant_pool(&self) -> usize;
}
