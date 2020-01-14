use crate::runtime_data_area::heap::access_flags::AccessFlag::*;

pub enum AccessFlag {
    Public = 0x0001,
    Private(i32),
    Protected(i32),
    Static(i32),
    Final(i32),
    Super(i32),
    Synchronized(i32),
    Volatile(i32),
    Bridge(i32),
    Transient(i32),
    Varargs(i32),
    Native(i32),
    Interface(i32),
    Abstract(i32),
    Strict(i32),
    Synthetic(i32),
    Annotation(i32),
    Enum(i32)
}

pub const PUBLIC:u16 = 0x0001;
pub const PRIVATE:u16 = 0x0002;
pub const PROTECTED:u16 = 0x0004;
pub const STATIC:u16 = 0x0008;
pub const FINAL:u16 = 0x0010;
pub const SUPER:u16 = 0x0020;
pub const SYNCHRONIZED:u16 = 0x0020;
pub const VOLATILE:u16 = 0x0040;
pub const BRIDGE:u16 = 0x0040;
pub const TRANSIENT:u16 = 0x0080;
pub const VARARGS:u16 = 0x0080;
pub const NATIVE:u16 = 0x0100;
pub const INTERFACE:u16 = 0x0200;
pub const ABSTRACT:u16 = 0x0400;
pub const STRICT:u16 = 0x0800;
pub const SYNTHETIC:u16 = 0x1000;
pub const ANNOTATION:u16 = 0x2000;
pub const ENUM:u16 = 0x4000;