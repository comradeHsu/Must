use crate::oops::class::Class;
use crate::oops::field::Field;
use crate::oops::member_ref::MemberRef;
use lark_classfile::constant_pool::ConstantFieldRefInfo;
use std::sync::RwLock;

pub struct FieldRef {
    member_ref: MemberRef,
    field: RwLock<Option<Field>>,
}

impl FieldRef {
    pub fn new_field_ref(info: &ConstantFieldRefInfo) -> FieldRef {
        let mut field_ref = FieldRef {
            member_ref: MemberRef::new(),
            field: RwLock::new(None),
        };
        field_ref.member_ref.copy_member_info(info.get_member_ref());
        return field_ref;
    }

    pub fn resolved_field(&self, holder: &Class) -> Field {
        let field_op = {
            let field = self.field.read().unwrap();
            field.clone()
        };
        match field_op {
            Some(field) => field,
            None => self.resolve_field_ref(holder)
        }
    }

    // jvms 5.4.3.2
    fn resolve_field_ref(&self, holder: &Class) -> Field {
        let resolved_class = self.member_ref.resolved_class(holder);
        let field = FieldRef::lookup_field(
            &resolved_class,
            self.member_ref.name(),
            self.member_ref.descriptor(),
        );
        if field.is_none() {
            panic!("java.lang.NoSuchFieldError");
        }
        let rc_field = field.unwrap();
        if !rc_field
            .is_accessible_to(holder)
        {
            panic!("java.lang.IllegalAccessError")
        }
        let mut field = self.field.write().unwrap();
        *field = Some(rc_field.clone());
        rc_field
    }

    fn lookup_field(
        class: &Class,
        name: &str,
        descriptor: &str,
    ) -> Option<Field> {
        let field = class.fields_with(|fields|{
            for field in fields {
                if field.name() == name
                    && field.descriptor() == descriptor
                {
                    return Some(field.clone())
                }
            }
            None
        });
        if field.is_some() {
            return field;
        }
        if let Some(v) = class.interfaces_with(|interfaces|{
            if interfaces.is_some() {
                for interface in interfaces.unwrap() {
                    let field = FieldRef::lookup_field(interface, name, descriptor);
                    if field.is_some() {
                        return field;
                    }
                }
            }
            None
        }) {
            return Some(v)
        }
        let super_class = class.super_class();
        if super_class.is_some() {
            return FieldRef::lookup_field(&super_class.unwrap(), name, descriptor);
        }
        return None;
    }

    pub fn name(&self) -> &str {
        return self.member_ref.name();
    }
}
