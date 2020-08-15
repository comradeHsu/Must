use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::instruction::Instruction;
use crate::runtime::frame::Frame;
use crate::oops::array_object::ArrayObject;
use crate::oops::class::Class;
use crate::oops::constant_pool::Constant::ClassReference;
use crate::utils::boxed;
use std::cell::RefCell;
use std::rc::Rc;
use crate::instructions::references::ResolveClassRef;

pub struct MultiANewArray {
    index: u16,
    dimensions: u8,
}

impl MultiANewArray {
    #[inline]
    pub fn new() -> MultiANewArray {
        return MultiANewArray {
            index: 0,
            dimensions: 0,
        };
    }

    fn pop_and_check_counts(frame: &Frame, dimensions: usize) -> Vec<i32> {
        frame.operand_stack(move |stack| {
            let mut counts = Vec::with_capacity(dimensions);
            for dimension in 1..dimensions {
                let index = dimensions - dimension;
                counts[index] = stack.pop_int();
                if counts[index] < 0 {
                    panic!("java.lang.NegativeArraySizeException")
                }
            }
            return counts;
        })
    }

    fn new_multi_dimensional_array(
        mut counts: Vec<i32>,
        arr_class: Rc<RefCell<Class>>,
    ) -> ArrayObject {
        let count = counts[0] as usize;
        let mut arr = Class::new_array(&arr_class, count);
        if counts.len() > 1 {
            let refs = arr.mut_references();
            for i in 0..refs.len() {
                refs[i] = Some(boxed(MultiANewArray::new_multi_dimensional_array(
                    counts.split_off(1),
                    (*arr_class).borrow().component_class(),
                )));
            }
        }
        return arr;
    }
}

impl Instruction for MultiANewArray {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16();
        self.dimensions = reader.read_u8();
    }

    fn execute(&mut self, frame: &Frame) {
        let current_class = frame.method().class();

        let array_class = self.resolve_class_ref(current_class);
        let counts = MultiANewArray::pop_and_check_counts(frame, self.dimensions as usize);
        let arr = MultiANewArray::new_multi_dimensional_array(counts, array_class);
        frame
            .push_ref(Some(boxed(arr)));
    }
}

impl ResolveClassRef for MultiANewArray {
    fn get_index(&self) -> usize {
        return self.index as usize;
    }
}
