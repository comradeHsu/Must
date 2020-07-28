use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::method::Method;
use crate::runtime_data_area::heap::constant_pool::Constant::{MethodReference, FieldReference, ClassReference, InterfaceMethodReference};
use std::ops::Deref;
use crate::runtime_data_area::heap::constant_pool::Constant;

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
        let constant = (*class)
            .borrow_mut()
            .mut_constant_pool()
            .take_constant(self.get_index());
        let mut field_ref = match constant {
            FieldReference(refe) => refe,
            _ => panic!("Unknown constant type"),
        };
        let field = field_ref.resolved_field(class.clone()).unwrap();
        (*class)
            .borrow_mut()
            .mut_constant_pool()
            .restoration_constant(self.get_index(),Constant::FieldReference(field_ref));
        return field;
    }

    fn get_index(&self) -> usize;
}

trait ResolveClassRef {
    fn resolve_class_ref(&self,class:Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        let constant = (*class)
            .borrow_mut()
            .mut_constant_pool()
            .take_constant(self.get_index());
        let mut class_ref = match constant {
            ClassReference(refe) => refe,
            _ => panic!("Unknown constant type"),
        };
        let resolved_class = class_ref.resolved_class(class.clone());
        (*class)
            .borrow_mut()
            .mut_constant_pool()
            .restoration_constant(self.get_index(),Constant::ClassReference(class_ref));
        return resolved_class;
    }

    fn get_index(&self) -> usize;
}

trait ResolveMethodRef {
    fn resolved_method_ref(&self,class:Rc<RefCell<Class>>) -> Rc<Method> {
        let constant = (*class)
            .borrow_mut()
            .mut_constant_pool()
            .take_constant(self.get_index());
        let mut method_ref = match constant {
            MethodReference(refe) => refe,
            _ => panic!("Unknown constant type"),
        };
        let method =  method_ref.resolved_method(class.clone());
        (*class)
            .borrow_mut()
            .mut_constant_pool()
            .restoration_constant(self.get_index(),Constant::MethodReference(method_ref));
        return method.unwrap();
    }

    fn resolved_method_ref_tuple(&self,class:Rc<RefCell<Class>>) -> (Rc<RefCell<Class>>,Rc<Method>) {
        let constant = (*class)
            .borrow_mut()
            .mut_constant_pool()
            .take_constant(self.get_index());
        let mut method_ref = match constant {
            MethodReference(refe) => refe,
            _ => panic!("Unknown constant type")
        };
        let method = method_ref.resolved_method(class.clone());
        let resolved_class = method_ref.resolved_class(class.clone());
        (*class)
            .borrow_mut()
            .mut_constant_pool()
            .restoration_constant(self.get_index(),Constant::MethodReference(method_ref));
        return (resolved_class,method.unwrap());
    }

    fn get_index(&self) -> usize;
}

trait ResolveInterfaceMethodRef {
    fn resolved_interface_method_ref(&self,class:Rc<RefCell<Class>>) -> Rc<Method> {
        let constant = (*class)
            .borrow_mut()
            .mut_constant_pool()
            .take_constant(self.get_index());
        let mut method_ref = match constant {
            InterfaceMethodReference(refe) => refe,
            _ => panic!("Unknown constant type"),
        };
        let method = method_ref.resolved_interface_method(class.clone());
        (*class)
            .borrow_mut()
            .mut_constant_pool()
            .restoration_constant(self.get_index(),Constant::InterfaceMethodReference(method_ref));
        return method.unwrap();
    }

    fn resolved_interface_method_ref_tuple(&self,class:Rc<RefCell<Class>>) -> (Rc<RefCell<Class>>,Rc<Method>) {
        let constant = (*class)
            .borrow_mut()
            .mut_constant_pool()
            .take_constant(self.get_index());
        let mut method_ref = match constant {
            InterfaceMethodReference(refe) => refe,
            _ => panic!("Unknown constant type")
        };
        let method = method_ref.resolved_interface_method(class.clone());
        let resolved_class = method_ref.resolved_class(class.clone());
        (*class)
            .borrow_mut()
            .mut_constant_pool()
            .restoration_constant(self.get_index(),Constant::InterfaceMethodReference(method_ref));
        return (resolved_class,method.unwrap());
    }

    fn get_index(&self) -> usize;
}
