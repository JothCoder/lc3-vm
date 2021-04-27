use super::utils;

pub const MEMORY_SIZE: usize = u16::MAX as _;

/// Address constants of the memory mapped registers
mod mem_mapped_reg_addr {
    /// Keyboard status register
    pub const KBSR: u16 = 0xFE00;
    /// Keyboard data register
    pub const KBDR: u16 = 0xFE02;
}

/// Wrapper type that represents the vm's memory
pub struct Memory {
    mem: [u16; MEMORY_SIZE],
}

impl Memory {
    /// Creates a new empty `Memory`
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_SIZE],
        }
    }

    /// Reads the value at the given memory `address`
    ///
    /// This requires a mutable reference to self, because reading a Memory Mapped Register may
    /// have side-effects.
    pub fn read(&mut self, address: u16) -> u16 {
        if address == mem_mapped_reg_addr::KBSR {
            let chr = utils::io::read_next_byte();
            if chr != 0 {
                self.mem[mem_mapped_reg_addr::KBSR as usize] = 1 << 15;
                self.mem[mem_mapped_reg_addr::KBDR as usize] = chr as u16;
            } else {
                self.mem[mem_mapped_reg_addr::KBSR as usize] = 0;
            }
        }
        self.mem[address as usize]
    }

    /// Writes the `value` to the given memory `address`
    pub fn write(&mut self, address: u16, value: u16) {
        self.mem[address as usize] = value;
    }
}
