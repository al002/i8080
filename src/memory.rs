// use std::ops::{Index, Range, RangeFrom};
// use crate::pointer::Pointer;

pub struct Memory {
    pub memory: Vec<u8>,
    pub ram_mirror: Option<u16>,
}

impl Memory {
    pub fn load(&mut self, block: &[u8], position: u16) {
        for (byte, pos) in block.iter().zip(position..) {
            self.memory[pos as usize] = *byte;
        }
    }

    pub fn write<T: Into<u16>, U: Into<u8>>(&mut self, address: T, value: U) {
        let address = address.into();
        let value = value.into();

        match self.ram_mirror {
            Some(mirror) if address >= mirror || address < 0x2000 => {},
            _ => self.memory[address as usize] = value,
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: vec![0; 0x10000],
            ram_mirror: None,
        }
    }
}
