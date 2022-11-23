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

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4}",
                    "a",   "bc", "de", "hl", "pc", "sp", "flags")?;

        write!(f,
                 "{:04x} {:02x}{:02x} {:02x}{:02x} {:02x}{:02x} {:04x} {:04x} {:?}",
                 *self.a,
                 *self.b, *self.c,
                 *self.d, *self.e,
                 *self.h, *self.l,
                 *self.pc,
                 *self.sp,
                 self.conditions,
        )
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self::default() 
    }

    pub fn load_into_rom(&mut self, memory: &[u8], position: u16) {
        self.memory.load(memory, position)
    }

    pub fn execute<M: Machine>(&mut self, machine: &mut M) -> u8 {
        let pc = self.pc;
        let opcode = Opcode::from(self.memory[pc]);
        let mut jumped = false;

        // match *opcode {
        //
        // }
        
        if !jumped {
            self.pc += opcode.size() as u16;
        }

        opcode.cycle_size()
    }
    
    pub fn print_opcode(&self) {
        let pc = self.pc;
        let opcode = Opcode::from(self.memory[pc]);

        if opcode.size() == 1 {
            println!("{:04x} {:?}", *pc, opcode);
        } else if opcode.size() == 2 {
            println!("{:04x} {:?} {:02x}", *pc, opcode, self.memory[pc + 1]);
        } else {
            println!("{:04x} {:?} {:02x}{:02x}",
                     *pc,
                     opcode,
                     self.memory[pc+2],
                     self.memory[pc+1]);
        }
    }

    fn get_offset(&self) -> u8 {
        let offset = ((self.h.to_u16() << 8)) | self.l.to_u16();
        self.memory[offset]
    }

    fn set_offset<I: Into<u8>>(&mut self, value: I) {
        let offset = ((self.h.to_u16() << 8)) | self.l.to_u16();
        self.memory.write(offset, value);
    }

    fn get_d8(&self) -> u8 {
        self.memory[self.pc + 1]
    }

    fn get_d16(&self) -> u16 {
        (self.memory[self.pc + 2] as u16) << 8 | self.memory[self.pc + 1] as u16
    }
}

impl Cpu {
   fn mov(&mut self, code: u8) {
        macro_rules! mov {
            ($set:ident ,
             $b:expr ,
             $c:expr ,
             $d:expr ,
             $e:expr ,
             $h:expr ,
             $l:expr ,
             $hl:expr ,
             $a:expr) => {
                self.$set = match code {
                     $b => self.b,
                     $c => self.c,
                     $d => self.d,
                     $e => self.e,
                     $h => self.h,
                     $l => self.l,
                     $hl => self.get_offset().into(),
                     $a => self.a,
                     _ => unreachable!(),
                 }
             }
        }

        macro_rules! movm {
            ($b:expr ,
             $c:expr ,
             $d:expr ,
             $e:expr ,
             $h:expr ,
             $l:expr ,
             $a:expr) => {{
                 let content = match code {
                     $b => self.b,
                     $c => self.c,
                     $d => self.d,
                     $e => self.e,
                     $h => self.h,
                     $l => self.l,
                     $a => self.a,
                     _ => unreachable!(),
                 };

                 self.set_offset(content);
             }}
        }

        match code {
            0x40..=0x47 => mov!(b, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47),
            0x48..=0x4f => mov!(c, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f),
            0x50..=0x57 => mov!(d, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57),
            0x58..=0x5f => mov!(e, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f),
            0x60..=0x67 => mov!(h, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67),
            0x68..=0x6f => mov!(l, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f),
            0x70..=0x75 | 0x77 => movm!(0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x77),
            0x78..=0x7f => mov!(a, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f),
            _ => unreachable!(),
        }
    }

    fn mvi(&mut self, code: u8) {
        macro_rules! mvi {
            ($x:ident) => {{
                self.$x = self.get_d8().into();
            }}
        }

        match code {
            0x06 => mvi!(b),
            0x0e => mvi!(c),
            0x16 => mvi!(d),
            0x1e => mvi!(e),
            0x26 => mvi!(h),
            0x2e => mvi!(l),
            0x36 => {
                let byte = self.get_d8();
                self.set_offset(byte);
            },
            0x3e => mvi!(a),
            _ => unreachable!(),
        }
    }

    fn lxi(&mut self, code: u8) {
        macro_rules! lxi {
            ($x:ident  $y:ident) => {{
                self.$x = (self.get_d16() >> 8).into();
                self.$y = self.get_d8().into();
            }}
        }

        match code {
            0x01 => lxi!(b c),
            0x11 => lxi!(d e),
            0x21 => lxi!(h l),
            0x31 => {
                self.sp = self.get_d16().into()
            },
            _ => unreachable!(),
        }
    }
}
