use crate::runtime_data_area::heap::object::{Object, DataType};
use crate::runtime_data_area::heap::object::DataType::{Bytes, Shorts, Ints, Longs, Chars, Floats, Doubles, References};
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime_data_area::heap::class::Class;

pub type ArrayObject = Object;

impl ArrayObject {

    #[inline]
    pub fn from_data(class:Rc<RefCell<Class>>,data:DataType) -> ArrayObject {
        return Object{
            class,
            data
        };
    }

    pub fn bytes(&self) -> &Vec<i8> {
        match &self.data {
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

    pub fn ints(&self) -> &Vec<i32> {
        match &self.data {
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

    pub fn chars(&self) -> &Vec<u16> {
        match &self.data {
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

    pub fn doubles(&self) -> &Vec<f64> {
        match &self.data {
            Doubles(array) => array,
            _ => panic!("The object type is error")
        }
    }

    pub fn references(&self) -> &Vec<Rc<RefCell<Object>>> {
        match &self.data {
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
}