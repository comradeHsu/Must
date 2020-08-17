use crate::oops::class::Class;
use crate::oops::object::DataType::{
    Bytes, Chars, Doubles, Floats, Ints, Longs, References, Shorts,
};
use crate::oops::object::{Data, DataType, MetaData, Object};
use std::cell::RefCell;
use std::rc::Rc;

pub type ArrayObject = Object;

impl Object {
    #[inline]
    pub fn from_data(class: Rc<RefCell<Class>>, data: DataType) -> ArrayObject {
        return Object {
            data: Rc::new(RefCell::new(Data {
                class,
                data,
                meta_data: MetaData::Null,
            })),
        };
    }

    pub fn bytes<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<i8>) -> R,
    {
        match &(*self.data).borrow().data {
            Bytes(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_bytes<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<i8>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            Bytes(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn shorts<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<i16>) -> R,
    {
        match &(*self.data).borrow().data {
            Shorts(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_shorts<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<i16>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            Shorts(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn ints<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<i32>) -> R,
    {
        match &(*self.data).borrow().data {
            Ints(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_ints<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<i32>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            Ints(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn longs<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<i64>) -> R,
    {
        match &(*self.data).borrow().data {
            Longs(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_longs<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<i64>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            Longs(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn get_long_by_index(&self, index: usize) -> i64 {
        self.longs(|array| array.get(index).map_or_else(|| 0, |x| *x))
    }

    pub fn set_long_by_index(&self, index: usize, value: i64) {
        self.mut_longs(|array| array[index] = value)
    }

    pub fn chars<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<u16>) -> R,
    {
        match &(*self.data).borrow().data {
            Chars(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_chars<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<u16>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            Chars(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn floats<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<f32>) -> R,
    {
        match &(*self.data).borrow().data {
            Floats(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_floats<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<f32>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            Floats(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn doubles<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<f64>) -> R,
    {
        match &(*self.data).borrow().data {
            Doubles(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_doubles<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<f64>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            Doubles(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn references<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&Vec<Option<Object>>) -> R,
    {
        match &(*self.data).borrow().data {
            References(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn mut_references<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&mut Vec<Option<Object>>) -> R,
    {
        match &mut (*self.data).borrow_mut().data {
            References(array) => func(array),
            _ => panic!("The object type is error"),
        }
    }

    pub fn get_references_by_index(&self, index: usize) -> Option<Object> {
        self.references(|array| array.get(index).map_or_else(|| None, |x| x.clone()))
    }

    pub fn array_length(&self) -> usize {
        match &(*self.data).borrow().data {
            Bytes(array) => array.len(),
            Shorts(array) => array.len(),
            Ints(array) => array.len(),
            Longs(array) => array.len(),
            Chars(array) => array.len(),
            Floats(array) => array.len(),
            Doubles(array) => array.len(),
            References(array) => array.len(),
            _ => panic!("The object isn't array"),
        }
    }

    pub fn array_copy(src: Object, dst: Object, src_pos: usize, dst_pos: usize, length: usize) {
        if src == dst {
            ArrayObject::array_copy_from_same_object(src, src_pos, dst_pos, length);
            return;
        }
        dst.mut_data_with(move |data| {
            src.data_with(|src_data| match (src_data, data) {
                (Bytes(s), Bytes(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.copy_from_slice(s_slice);
                }
                (Shorts(s), Shorts(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.copy_from_slice(s_slice);
                }
                (Ints(s), Ints(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.copy_from_slice(s_slice);
                }
                (Longs(s), Longs(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.copy_from_slice(s_slice);
                }
                (Chars(s), Chars(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.copy_from_slice(s_slice);
                }
                (Floats(s), Floats(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.copy_from_slice(s_slice);
                }
                (Doubles(s), Doubles(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.copy_from_slice(s_slice);
                }
                (References(s), References(d)) => {
                    let s_slice = &s[src_pos..(src_pos + length)];
                    let d_slice = &mut d[dst_pos..(dst_pos + length)];
                    d_slice.clone_from_slice(s_slice);
                }
                _ => panic!("The object isn't array"),
            });
        });
    }

    fn array_copy_from_same_object(object: Object, src_pos: usize, dst_pos: usize, length: usize) {
        object.mut_data_with(|data| match data {
            Bytes(s) => {
                s.copy_within(src_pos..(src_pos + length), dst_pos);
            }
            Shorts(s) => {
                s.copy_within(src_pos..(src_pos + length), dst_pos);
            }
            Ints(s) => {
                s.copy_within(src_pos..(src_pos + length), dst_pos);
            }
            Longs(s) => {
                s.copy_within(src_pos..(src_pos + length), dst_pos);
            }
            Chars(s) => {
                s.copy_within(src_pos..(src_pos + length), dst_pos);
            }
            Floats(s) => {
                s.copy_within(src_pos..(src_pos + length), dst_pos);
            }
            Doubles(s) => {
                s.copy_within(src_pos..(src_pos + length), dst_pos);
            }
            References(s) => {
                for i in 0..length {
                    s[dst_pos + i] = s[src_pos + i].clone();
                }
            }
            _ => panic!("The object isn't array"),
        });
    }
}
