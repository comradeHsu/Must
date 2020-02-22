use crate::class_file::constant_pool::ConstantFieldRefInfo;
use crate::runtime_data_area::heap::class::Class;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::member_ref::MemberRef;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct FieldRef {
    member_ref: MemberRef,
    field: Option<Rc<RefCell<Field>>>,
}

impl FieldRef {
    pub fn new_field_ref(info: &ConstantFieldRefInfo) -> FieldRef {
        let mut field_ref = FieldRef {
            member_ref: MemberRef::new(),
            field: None,
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }

    pub fn resolved_field(&mut self) -> Option<Rc<RefCell<Field>>> {
        if self.field.is_none() {
            self.resolve_field_ref()
        }
        return self.field.clone();
    }

    // jvms 5.4.3.2
    fn resolve_field_ref(&mut self) {
        let resolved_class = self.member_ref.resolved_class();
        let field = FieldRef::lookup_field(
            &resolved_class,
            self.member_ref.name(),
            self.member_ref.descriptor(),
        );
        if field.is_none() {
            panic!("java.lang.NoSuchFieldError");
        }
        let rc_field = field.unwrap().clone();
        if !(*rc_field)
            .borrow()
            .is_accessible_to((*self.member_ref.holder()).borrow().deref())
        {
            panic!("java.lang.IllegalAccessError")
        }

        self.field = Some(rc_field);
    }

    fn lookup_field(
        class: &Rc<RefCell<Class>>,
        name: &str,
        descriptor: &str,
    ) -> Option<Rc<RefCell<Field>>> {
        let class = class.clone();
        for field in (*class).borrow().fields() {
            let rc_field = field.clone();
            if (*rc_field).borrow().name() == name
                && (*rc_field).borrow().descriptor() == descriptor
            {
                return Some(field.clone());
            }
        }
        let borrow_class = (*class).borrow();
        let interfaces = borrow_class.interfaces();
        if interfaces.is_some() {
            for interface in interfaces.unwrap() {
                let field = FieldRef::lookup_field(interface, name, descriptor);
                if field.is_some() {
                    return field;
                }
            }
        }
        let borrow_class = (*class).borrow();
        let super_class = borrow_class.super_class();
        if super_class.is_some() {
            return FieldRef::lookup_field(&super_class.unwrap(), name, descriptor);
        }
        return None;
    }

    #[inline]
    pub fn set_holder(&mut self, holder: Rc<RefCell<Class>>) {
        self.member_ref.set_holder(holder);
    }
}
