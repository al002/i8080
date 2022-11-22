use std::{fmt, mem};

use crate::condition_codes::ConditionCodes;
use crate::memory::Memory;
use crate::opcode::Opcode;
use crate::register::Register;
use crate::pointer::Pointer;

#[derive(Default)]
pub struct Cpu {
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub h: Register,
    pub l: Register,
    pub sp: Pointer,
    pub pc: Pointer,
    pub memory: Memory,
    pub conditions: ConditionCodes,
    pub int_enable: bool,
}

pub trait Machine {
    fn input(&mut self, port: u8) -> u8;
    fn output(&mut self, port: u8, byte: u8);
}

// impl fmt::Debug for Cpu {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "{:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4}",
//                     "a",   "bc", "de", "hl", "pc", "sp", "flags")?;
//
//         write!(f,
//                  "{:04x} {:02x}{:02x} {:02x}{:02x} {:02x}{:02x} {:04x} {:04x} {:?}",
//                  *self.a,
//                  *self.b, *self.c,
//                  *self.d, *self.e,
//                  *self.h, *self.l,
//                  *self.pc,
//                  *self.sp,
//                  self.conditions,
//         )
//     }
// }
