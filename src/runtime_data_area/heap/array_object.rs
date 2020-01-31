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
        let mut src_borrow = (*src).borrow();
        let mut dst_borrow = (*dst).borrow_mut();
        match (src_borrow.data(), dst_borrow.mut_data()){
            (Bytes(s),Bytes(d) )=> {
                let slice = &s[src_pos..(src_pos+length)];
                let dlice = &mut d[dst_pos..(dst_pos+length)];
                dlice.copy_from_slice(slice);
            },
            (Shorts(s),Shorts(d)) => {},
            (Ints(s),Ints(d)) => {},
            (Longs(s),Longs(d)) => {},
            (Chars(s),Chars(d)) => {},
            (Floats(s),Floats(d)) => {},
            (Doubles(s),Doubles(d)) => {},
            (References(s),References(d)) => {},
            _ => panic!("The object isn't array")
        }
    }
}