use crate::oops::class::Class;
use crate::oops::constant_pool::Constant;
use crate::oops::constant_pool::Constant::{
    ClassReference, FieldReference, InterfaceMethodReference, MethodReference,
};
use crate::oops::field::Field;
use crate::oops::method::Method;

use std::cell::RefCell;

use std::rc::Rc;

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
    fn resolve_field_ref(&self, class: &Class) -> Field {
        class.constant_with(self.get_index(),|constant|{
            let field_ref = match constant {
                FieldReference(refe) => refe,
                _ => panic!("Unknown constant type"),
            };
            let field = field_ref.resolved_field(class);
            field
        })
    }

    fn get_index(&self) -> usize;
}

trait ResolveClassRef {
    fn resolve_class_ref(&self, class: &Class) -> Class {
        class.constant_with(self.get_index(),|constant|{
            let class_ref = match constant {
                ClassReference(refe) => refe,
                _ => panic!("Unknown constant type"),
            };
            let resolved_class = class_ref.resolved_class(class);
            resolved_class
        })
    }

    fn get_index(&self) -> usize;
}

trait ResolveMethodRef {
    fn resolved_method_ref(&self, class: &Class) -> Method {
        class.constant_with(self.get_index(),|constant|{
            let method_ref = match constant {
                MethodReference(refe) => refe,
                _ => panic!("Unknown constant type"),
            };
            let method = method_ref.resolved_method(class);
            method
        })
    }

    fn resolved_method_ref_tuple(
        &self,
        class: &Class,
    ) -> (Class, Method) {
        class.constant_with(self.get_index(),|constant|{
            let method_ref = match constant {
                MethodReference(refe) => refe,
                _ => panic!("Unknown constant type"),
            };
            let method = method_ref.resolved_method(class);
            let resolved_class = method_ref.resolved_class(class);
            (resolved_class, method)
        })
    }

    fn get_index(&self) -> usize;
}

trait ResolveInterfaceMethodRef {
    fn resolved_interface_method_ref(&self, class: &Class) -> Method {
        class.constant_with(self.get_index(),|constant|{
            let method_ref = match constant {
                InterfaceMethodReference(refe) => refe,
                _ => panic!("Unknown constant type"),
            };
            let method = method_ref.resolved_interface_method(class);
            method
        })
    }

    fn resolved_interface_method_ref_tuple(
        &self,
        class: &Class,
    ) -> (Class, Method) {
        class.constant_with(self.get_index(),|constant|{
            let method_ref = match constant {
                InterfaceMethodReference(refe) => refe,
                _ => panic!("Unknown constant type"),
            };
            let method = method_ref.resolved_interface_method(class);
            let resolved_class = method_ref.resolved_class(class);
            (resolved_class, method)
        })
    }

    fn get_index(&self) -> usize;
}
