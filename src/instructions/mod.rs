use crate::instructions::base::instruction::Instruction;
use crate::instructions::constants::nop::Nop;
use crate::instructions::constants::constant::{AconstNull, IconstM1, Iconst0, Iconst1, Iconst2,
                                               Iconst3, Iconst4, Iconst5, Lconst0, Lconst1, Fconst0,
                                               Fconst1, Fconst2, Dconst0, Dconst1};
use crate::instructions::constants::ipush::{BiPush, SiPush};
use crate::instructions::loads::iload::{ILoad, ILoad0, ILoad1, ILoad2, ILoad3};
use crate::instructions::loads::lload::{LLoad, LLoad0, LLoad1, LLoad2, LLoad3};
use crate::instructions::loads::fload::{FLoad, FLoad0, FLoad1, FLoad2, FLoad3};
use crate::instructions::loads::dload::{DLoad, DLoad0, DLoad1, DLoad2, DLoad3};
use crate::instructions::loads::aload::{ALoad, ALoad0, ALoad1, ALoad2, ALoad3};
use crate::instructions::stores::istore::{IStore, IStore0, IStore1, IStore2, IStore3};
use crate::instructions::stores::lstore::{LStore, LStore0, LStore1, LStore2, LStore3};
use crate::instructions::stores::fstore::{FStore, FStore0, FStore1, FStore2, FStore3};
use crate::instructions::stores::dstore::{DStore, DStore0, DStore1, DStore2, DStore3};
use crate::instructions::stores::astore::{AStore, AStore0, AStore1, AStore2, AStore3};
use crate::instructions::stack::pop::{Pop, Pop2};
use crate::instructions::stack::dup::{Dup, DupX1, DupX2, Dup2, Dup2X1, Dup2X2};
use crate::instructions::stack::swap::Swap;
use crate::instructions::math::add::{IAdd, LAdd, FAdd, DAdd};
use crate::instructions::math::sub::{ISub, LSub, FSub, DSub};
use crate::instructions::math::mul::{IMul, LMul, FMul, DMul};
use crate::instructions::math::div::{IDiv, LDiv, FDiv, DDiv};
use crate::instructions::math::rem::{IRem, LRem, FRem, DRem};
use crate::instructions::math::neg::{INeg, LNeg, FNeg, DNeg};
use crate::instructions::math::sh::{IShl, LShl, IShr, LShr, IuShr, LuShr};
use crate::instructions::math::and::{IAnd, LAnd};
use crate::instructions::math::or::{IOr, LOr};
use crate::instructions::math::xor::{IXor, LXor};
use crate::instructions::math::iinc::IInc;
use crate::instructions::conversions::i2x::{I2l, I2d, I2f, I2b, I2c, I2s};
use crate::instructions::conversions::l2x::{L2i, L2f, L2d};
use crate::instructions::conversions::f2x::{F2i, F2l, F2d};
use crate::instructions::conversions::d2x::{D2i, D2l, D2f};
use crate::instructions::comparisons::lcmp::Lcmp;
use crate::instructions::comparisons::fcmp::{Fcmpl, Fcmpg};
use crate::instructions::comparisons::dcmp::{Dcmpl, Dcmpg};
use crate::instructions::comparisons::ifcond::{IfEq, IfNe, IfLt, IfGe, IfGt, IfLe};
use crate::instructions::comparisons::if_icmp::{IfICmpEq, IfICmpNe, IfICmpGt, IfICmpLt, IfICmpGe,
                                                IfICmpLe};
use crate::instructions::comparisons::if_acmp::{IfACmpEq, IfACmpNe};
use crate::instructions::control::goto::Goto;
use crate::instructions::control::table_switch::TableSwitch;
use crate::instructions::control::lookup_switch::{LookUpSwitch};
use crate::instructions::extended::wide::Wide;
use crate::instructions::extended::ifnull::{IfNull, IfNonNull};
use crate::instructions::extended::goto_w::GotoW;

pub mod base;
mod constants;
mod loads;
mod stores;
mod stack;
mod math;
mod conversions;
mod comparisons;
mod control;
mod extended;
mod references;

pub fn new_instruction(opcode:u8) -> Box<dyn Instruction> {
    let inst:Box<dyn Instruction> = match opcode {
        0x00 => Box::new(Nop::new()),
        0x01 => Box::new(AconstNull::new()),
        0x02 => Box::new(IconstM1::new()),
        0x03 => Box::new(Iconst0::new()),
        0x04 => Box::new(Iconst1::new()),
        0x05 => Box::new(Iconst2::new()),
        0x06 => Box::new(Iconst3::new()),
        0x07 => Box::new(Iconst4::new()),
        0x08 => Box::new(Iconst5::new()),
        0x09 => Box::new(Lconst0::new()),
        0x0a => Box::new(Lconst1::new()),
        0x0b => Box::new(Fconst0::new()),
        0x0c => Box::new(Fconst1::new()),
        0x0d => Box::new(Fconst2::new()),
        0x0e => Box::new(Dconst0::new()),
        0x0f => Box::new(Dconst1::new()),
        0x10 => Box::new(BiPush::new()),
        0x11 => Box::new(SiPush::new()),
//        0x12 => {},
//        0x13 => {},
//        0x14 => {},
        0x15 => Box::new(ILoad::new()),
        0x16 => Box::new(LLoad::new()),
        0x17 => Box::new(FLoad::new()),
        0x18 => Box::new(DLoad::new()),
        0x19 => Box::new(ALoad::new()),
        0x1a => Box::new(ILoad0::new()),
        0x1b => Box::new(ILoad1::new()),
        0x1c => Box::new(ILoad2::new()),
        0x1d => Box::new(ILoad3::new()),
        0x1e => Box::new(LLoad0::new()),
        0x1f => Box::new(LLoad1::new()),
        0x20 => Box::new(LLoad2::new()),
        0x21 => Box::new(LLoad3::new()),
        0x22 => Box::new(FLoad0::new()),
        0x23 => Box::new(FLoad1::new()),
        0x24 => Box::new(FLoad2::new()),
        0x25 => Box::new(FLoad3::new()),
        0x26 => Box::new(DLoad0::new()),
        0x27 => Box::new(DLoad1::new()),
        0x28 => Box::new(DLoad2::new()),
        0x29 => Box::new(DLoad3::new()),
        0x2a => Box::new(ALoad0::new()),
        0x2b => Box::new(ALoad1::new()),
        0x2c => Box::new(ALoad2::new()),
        0x2d => Box::new(ALoad3::new()),
//        0x2e => {},
//        0x2f => {},
//        0x30 => {},
//        0x31 => {},
//        0x32 => {},
//        0x33 => {},
//        0x34 => {},
//        0x35 => {},
        0x36 => Box::new(IStore::new()),
        0x37 => Box::new(LStore::new()),
        0x38 => Box::new(FStore::new()),
        0x39 => Box::new(DStore::new()),
        0x3a => Box::new(AStore::new()),
        0x3b => Box::new(IStore0::new()),
        0x3c => Box::new(IStore1::new()),
        0x3d => Box::new(IStore2::new()),
        0x3e => Box::new(IStore3::new()),
        0x3f => Box::new(LStore0::new()),
        0x40 => Box::new(LStore1::new()),
        0x41 => Box::new(LStore2::new()),
        0x42 => Box::new(LStore3::new()),
        0x43 => Box::new(FStore0::new()),
        0x44 => Box::new(FStore1::new()),
        0x45 => Box::new(FStore2::new()),
        0x46 => Box::new(FStore3::new()),
        0x47 => Box::new(DStore0::new()),
        0x48 => Box::new(DStore1::new()),
        0x49 => Box::new(DStore2::new()),
        0x4a => Box::new(DStore3::new()),
        0x4b => Box::new(AStore0::new()),
        0x4c => Box::new(AStore1::new()),
        0x4d => Box::new(AStore2::new()),
        0x4e => Box::new(AStore3::new()),
//        0x4f => {},
//        0x50 => {},
//        0x51 => {},
//        0x52 => {},
//        0x53 => {},
//        0x54 => {},
//        0x55 => {},
//        0x56 => {},
        0x57 => Box::new(Pop::new()),
        0x58 => Box::new(Pop2::new()),
        0x59 => Box::new(Dup::new()),
        0x5a => Box::new(DupX1::new()),
        0x5b => Box::new(DupX2::new()),
        0x5c => Box::new(Dup2::new()),
        0x5d => Box::new(Dup2X1::new()),
        0x5e => Box::new(Dup2X2::new()),
        0x5f => Box::new(Swap::new()),
        0x60 => Box::new(IAdd::new()),
        0x61 => Box::new(LAdd::new()),
        0x62 => Box::new(FAdd::new()),
        0x63 => Box::new(DAdd::new()),
        0x64 => Box::new(ISub::new()),
        0x65 => Box::new(LSub::new()),
        0x66 => Box::new(FSub::new()),
        0x67 => Box::new(DSub::new()),
        0x68 => Box::new(IMul::new()),
        0x69 => Box::new(LMul::new()),
        0x6a => Box::new(FMul::new()),
        0x6b => Box::new(DMul::new()),
        0x6c => Box::new(IDiv::new()),
        0x6d => Box::new(LDiv::new()),
        0x6e => Box::new(FDiv::new()),
        0x6f => Box::new(DDiv::new()),
        0x70 => Box::new(IRem::new()),
        0x71 => Box::new(LRem::new()),
        0x72 => Box::new(FRem::new()),
        0x73 => Box::new(DRem::new()),
        0x74 => Box::new(INeg::new()),
        0x75 => Box::new(LNeg::new()),
        0x76 => Box::new(FNeg::new()),
        0x77 => Box::new(DNeg::new()),
        0x78 => Box::new(IShl::new()),
        0x79 => Box::new(LShl::new()),
        0x7a => Box::new(IShr::new()),
        0x7b => Box::new(LShr::new()),
        0x7c => Box::new(IuShr::new()),
        0x7d => Box::new(LuShr::new()),
        0x7e => Box::new(IAnd::new()),
        0x7f => Box::new(LAnd::new()),
        0x80 => Box::new(IOr::new()),
        0x81 => Box::new(LOr::new()),
        0x82 => Box::new(IXor::new()),
        0x83 => Box::new(LXor::new()),
        0x84 => Box::new(IInc::new()),
        0x85 => Box::new(I2l::new()),
        0x86 => Box::new(I2f::new()),
        0x87 => Box::new(I2d::new()),
        0x88 => Box::new(L2i::new()),
        0x89 => Box::new(L2f::new()),
        0x8a => Box::new(L2d::new()),
        0x8b => Box::new(F2i::new()),
        0x8c => Box::new(F2l::new()),
        0x8d => Box::new(F2d::new()),
        0x8e => Box::new(D2i::new()),
        0x8f => Box::new(D2l::new()),
        0x90 => Box::new(D2f::new()),
        0x91 => Box::new(I2b::new()),
        0x92 => Box::new(I2c::new()),
        0x93 => Box::new(I2s::new()),
        0x94 => Box::new(Lcmp::new()),
        0x95 => Box::new(Fcmpl::new()),
        0x96 => Box::new(Fcmpg::new()),
        0x97 => Box::new(Dcmpl::new()),
        0x98 => Box::new(Dcmpg::new()),
        0x99 => Box::new(IfEq::new()),
        0x9a => Box::new(IfNe::new()),
        0x9b => Box::new(IfLt::new()),
        0x9c => Box::new(IfGe::new()),
        0x9d => Box::new(IfGt::new()),
        0x9e => Box::new(IfLe::new()),
        0x9f => Box::new(IfICmpEq::new()),
        0xa0 => Box::new(IfICmpNe::new()),
        0xa1 => Box::new(IfICmpLt::new()),
        0xa2 => Box::new(IfICmpGe::new()),
        0xa3 => Box::new(IfICmpGt::new()),
        0xa4 => Box::new(IfICmpLe::new()),
        0xa5 => Box::new(IfACmpEq::new()),
        0xa6 => Box::new(IfACmpNe::new()),
        0xa7 => Box::new(Goto::new()),
//        0xa8 => {},
//        0xa9 => {},
        0xaa => Box::new(TableSwitch::new()),
        0xab => Box::new(LookUpSwitch::new()),
//        0xac => {},
//        0xad => {},
//        0xae => {},
//        0xaf => {},
//        0xb0 => {},
//        0xb1 => {},
//        0xb2 => {},
//        0xb3 => {},
//        0xb4 => {},
//        0xb5 => {},
//        0xb6 => {},
//        0xb7 => {},
//        0xb8 => {},
//        0xb9 => {},
//        0xba => {},
//        0xbb => {},
//        0xbc => {},
//        0xbd => {},
//        0xbe => {},
//        0xbf => {},
//        0xc0 => {},
//        0xc1 => {},
//        0xc2 => {},
//        0xc3 => {},
        0xc4 => Box::new(Wide::new()),
//        0xc5 => {},
        0xc6 => Box::new(IfNull::new()),
        0xc7 => Box::new(IfNonNull::new()),
        0xc8 => Box::new(GotoW::new()),
        _c => {
            println!("opcode:{}",_c);
            panic!("instruction error")
        }
    };
    return inst;
}