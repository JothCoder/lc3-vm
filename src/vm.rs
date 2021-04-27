mod instructions;
mod memory;
mod opcode;
mod registers;
mod utils;

use memory::Memory;
use opcode::Opcode;
use registers::Registers;

use byteorder::{BigEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::{self, Read};

pub struct Vm {
    regs: Registers,
    mem: Memory,
    running: bool,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            mem: Memory::new(),
            running: false,
        }
    }

    pub fn load_program<R: Read>(&mut self, mut reader: R) -> io::Result<()> {
        let origin = reader.read_u16::<BigEndian>()?;
        for address in origin..(memory::MEMORY_SIZE as u16) {
            match reader.read_u16::<BigEndian>() {
                Ok(instr) => self.mem.write(address, instr),
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn run(&mut self) {
        let original_termios = utils::io::disable_input_buffering();
        self.running = true;
        self.main_loop();
        utils::io::restore_input_buffering(original_termios);
    }

    pub fn abort(&mut self) {
        self.running = false;
    }

    fn main_loop(&mut self) {
        while self.running {
            let instr = self.mem.read(self.regs.pc);
            self.regs.pc = self.regs.pc.wrapping_add(1);
            let (regs, mem) = (&mut self.regs, &mut self.mem);
            let opcode = Opcode::try_from(instr >> 12).unwrap();
            match opcode {
                Opcode::Br => instructions::br(instr, regs),
                Opcode::Add => instructions::add(instr, regs),
                Opcode::Ld => instructions::ld(instr, regs, mem),
                Opcode::St => instructions::st(instr, regs, mem),
                Opcode::Jsr => instructions::jsr(instr, regs),
                Opcode::And => instructions::and(instr, regs),
                Opcode::Ldr => instructions::ldr(instr, regs, mem),
                Opcode::Str => instructions::str(instr, regs, mem),
                Opcode::Rti => panic!("Illegal opcode: 0b1000 (RTI)"),
                Opcode::Not => instructions::not(instr, regs),
                Opcode::Ldi => instructions::ldi(instr, regs, mem),
                Opcode::Sti => instructions::sti(instr, regs, mem),
                Opcode::Jmp => instructions::jmp(instr, regs),
                Opcode::Res => panic!("Illegal opcode: 0b1101 (RES)"),
                Opcode::Lea => instructions::lea(instr, regs),
                Opcode::Trap => {
                    let should_halt = instructions::trap(instr, regs, mem);
                    if should_halt {
                        self.running = false;
                    }
                },
            };
        }
    }
}
