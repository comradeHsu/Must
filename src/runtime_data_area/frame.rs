use crate::runtime_data_area::local_vars::LocalVars;
use crate::runtime_data_area::operand_stack::OperandStack;

pub struct Frame {
    local_vars:Option<LocalVars>,
    operand_stack:Option<OperandStack>
}

impl Frame {
    #[inline]
    pub fn new() -> Frame {
        return Frame{
            local_vars: None,
            operand_stack: None
        };
    }
}