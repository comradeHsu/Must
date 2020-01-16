use crate::runtime_data_area::heap::member_ref::MemberRef;
use std::rc::Rc;
use crate::runtime_data_area::heap::field::Field;
use crate::runtime_data_area::heap::constant_pool::ConstantPool;
use crate::class_file::constant_pool::ConstantFieldRefInfo;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;
use std::borrow::Borrow;

pub struct FieldRef {
    member_ref:MemberRef,
    field:Option<Rc<RefCell<Field>>>
}

impl FieldRef {
    pub fn new_field_ref(cp:Rc<ConstantPool>,info:&ConstantFieldRefInfo) -> FieldRef {
        let mut field_ref = FieldRef{
            member_ref: MemberRef::with_pool(cp),
            field: None
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }

    pub fn resolved_field(&mut self) -> Option<&Rc<RefCell<Field>>> {
        if self.field.is_none(){
            self.resolve_field_ref()
        }
        return self.field.as_ref();
    }

    // jvms 5.4.3.2
    fn resolve_field_ref(&mut self) {
        let class = self.member_ref.constant_pool().class();
        let resolved_class = self.member_ref.resolved_class();
        let field = FieldRef::lookup_field(&resolved_class,
                                           self.member_ref.name(),
                                           self.member_ref.descriptor());
        if field.is_none(){
            panic!("java.lang.NoSuchFieldError");
        }
        if !field.unwrap().is_accessible_to((*class).borrow()) {
            panic!("java.lang.IllegalAccessError")
        }

        self.field = Some(field.unwrap().clone());
    }

    fn lookup_field(class: &Rc<RefCell<Class>>, name:&str, descriptor:&str) -> Option<&Rc<RefCell<Field>>> {
        for field in (*class).borrow().borrow().fields() {
            if field.name() == name && field.descriptor() == descriptor{
                return Some(field);
            }
        }
        let interfaces = (*class).borrow().borrow().interfaces();
        if interfaces.is_some() {
            for interface in interfaces.as_ref().unwrap() {
                let field = FieldRef::lookup_field(interface,name,descriptor);
                if field.is_some() {
                    return field;
                }
            }
        }
        let super_class =  (*class).borrow().borrow().super_class();
        if super_class.is_some() {
            return FieldRef::lookup_field(super_class.unwrap(),name,descriptor);
        }
        return None;
    }
}