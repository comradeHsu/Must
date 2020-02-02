use crate::runtime_data_area::heap::object::{Object, DataType};
use crate::runtime_data_area::heap::object::DataType::{Bytes, Shorts, Ints, Longs, Chars, Floats, Doubles, References};
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;

pub type ArrayObject = Object;

impl Object {

    #[inline]
    pub fn from_data(class:Rc<RefCell<Class>>,data:DataType) -> ArrayObject {
        return Object{
            class,
            data,
            meta: None
        };
    }

    pub fn bytes(&self) -> &Vec<i8> {
        match &self.data {
            Bytes(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_bytes(&mut self) -> &mut Vec<i8> {
        match &mut self.data {
            Bytes(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn shorts(&self) -> &Vec<i16> {
        match &self.data {
            Shorts(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_shorts(&mut self) -> &mut Vec<i16> {
        match &mut self.data {
            Shorts(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn ints(&self) -> &Vec<i32> {
        match &self.data {
            Ints(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_ints(&mut self) -> &mut Vec<i32> {
        match &mut self.data {
            Ints(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn longs(&self) -> &Vec<i64> {
        match &self.data {
            Longs(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_longs(&mut self) -> &mut Vec<i64> {
        match &mut self.data {
            Longs(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn chars(&self) -> &Vec<u16> {
        match &self.data {
            Chars(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_chars(&mut self) -> &mut Vec<u16> {
        match &mut self.data {
            Chars(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn floats(&self) -> &Vec<f32> {
        match &self.data {
            Floats(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_floats(&mut self) -> &mut Vec<f32> {
        match &mut self.data {
            Floats(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn doubles(&self) -> &Vec<f64> {
        match &self.data {
            Doubles(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_doubles(&mut self) -> &mut Vec<f64> {
        match &mut self.data {
            Doubles(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn references(&self) -> &Vec<Option<Rc<RefCell<Object>>>> {
        match &self.data {
            References(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn mut_references(&mut self) -> &mut Vec<Option<Rc<RefCell<Object>>>> {
        match &mut self.data {
            References(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn array_length(&self) -> usize {
        match &self.data {
            Bytes(array) => array.len(),
            Shorts(array) => array.len(),
            Ints(array) => array.len(),
            Longs(array) => array.len(),
            Chars(array) => array.len(),
            Floats(array) => array.len(),
            Doubles(array) => array.len(),
            References(array) => array.len(),
            _ => panic!("The object isn't array")
        }
    }

    pub fn array_copy(src:Rc<RefCell<Object>>,dst:Rc<RefCell<Object>>,src_pos:usize,dst_pos:usize,
                      length:usize) {
        if src == dst {
            ArrayObject::array_copy_from_same_object(src,src_pos,dst_pos,length);
            return;
        }
        let mut src_borrow = (*src).borrow();
        let mut dst_borrow = (*dst).borrow_mut();
        match (src_borrow.data(), dst_borrow.mut_data()){
            (Bytes(s),Bytes(d) )=> {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.copy_from_slice(s_slice);
            },
            (Shorts(s),Shorts(d)) => {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.copy_from_slice(s_slice);
            },
            (Ints(s),Ints(d)) => {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.copy_from_slice(s_slice);
            },
            (Longs(s),Longs(d)) => {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.copy_from_slice(s_slice);
            },
            (Chars(s),Chars(d)) => {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.copy_from_slice(s_slice);
            },
            (Floats(s),Floats(d)) => {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.copy_from_slice(s_slice);
            },
            (Doubles(s),Doubles(d)) => {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.copy_from_slice(s_slice);
            },
            (References(s),References(d)) => {
                let s_slice = &s[src_pos..(src_pos+length)];
                let d_slice = &mut d[dst_pos..(dst_pos+length)];
                d_slice.clone_from_slice(s_slice);
            },
            _ => panic!("The object isn't array")
        }
    }

    fn array_copy_from_same_object(object:Rc<RefCell<Object>>,src_pos:usize,dst_pos:usize, length:usize) {
        let mut dst_borrow = (*object).borrow_mut();
        match dst_borrow.mut_data() {
            Bytes(s)=> {
                s.copy_within(src_pos..(src_pos+length),dst_pos);
            },
            Shorts(s) => {
                s.copy_within(src_pos..(src_pos+length),dst_pos);
            },
            Ints(s) => {
                s.copy_within(src_pos..(src_pos+length),dst_pos);
            },
            Longs(s) => {
                s.copy_within(src_pos..(src_pos+length),dst_pos);
            },
            Chars(s) => {
                s.copy_within(src_pos..(src_pos+length),dst_pos);
            },
            Floats(s) => {
                s.copy_within(src_pos..(src_pos+length),dst_pos);
            },
            Doubles(s) => {
                s.copy_within(src_pos..(src_pos+length),dst_pos);
            },
            References(s) => {
                for i in 0..length {
                    s[dst_pos+i] = s[src_pos+i].clone();
                }
            },
            _ => panic!("The object isn't array")
        }
    }
}