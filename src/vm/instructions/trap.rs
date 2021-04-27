use crate::vm::{memory, utils, Memory, Registers};

use std::convert::TryFrom;
use std::io::{self, Write};

pub enum TrapCode {
    Getc,
    Out,
    Puts,
    In,
    Putsp,
    Halt,
}

impl TryFrom<u16> for TrapCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use TrapCode::*;

        let trap_code = match value {
            0x20 => Getc,
            0x21 => Out,
            0x22 => Puts,
            0x23 => In,
            0x24 => Putsp,
            0x25 => Halt,
            _ => return Err(()),
        };

        Ok(trap_code)
    }
}

pub fn getc(regs: &mut Registers) {
    regs.write(0, utils::io::read_next_byte() as u16);
}

pub fn out(regs: &Registers) {
    print!("{}", regs.read(0) as u8 as char);
    io::stdout().flush().expect("Error while flushing stdout");
}

pub fn puts(regs: &Registers, mem: &mut Memory) {
    for mem_addr in regs.read(0)..(memory::MEMORY_SIZE as u16) {
        let chr = mem.read(mem_addr);
        if chr == 0x0000 {
            break;
        }
        print!("{}", chr as u8 as char);
    }
    io::stdout().flush().expect("Error while flushing stdout");
}

pub fn putsp(regs: &Registers, mem: &mut Memory) {
    for mem_addr in regs.read(0)..(memory::MEMORY_SIZE as u16) {
        let word = mem.read(mem_addr);
        if word == 0x0000 {
            break;
        }
        let [chr2, chr1] = word.to_be_bytes();
        print!("{}{}", chr1 as char, chr2 as char);
    }
    io::stdout().flush().expect("Error while flushing stdout");
}

pub fn r#in(regs: &mut Registers) {
    print!("Enter character: ");
    regs.write(0, utils::io::read_next_byte() as u16);
}

pub fn halt() {
    print!("HALT");
    io::stdout().flush().expect("Error while flushing stdout");
}
