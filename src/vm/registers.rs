// Program Counter start
const PC_START: u16 = 0x3000;

pub struct Registers {
    /// Base Registers (R0..R7)
    base_regs: [u16; 8],
    /// Program Counter
    pub pc: u16,
    /// Condition Flags (NZP: Negative, Zero, Positive)
    pub cond: CondFlag,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum CondFlag {
    Pos = 0b001,
    Zero = 0b010,
    Neg = 0b100,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            base_regs: [0; 8],
            pc: PC_START,
            cond: CondFlag::Zero,
        }
    }

    /// Returns the value of the Base Register with the given index
    ///
    /// The `base_register_index` must be in the range 0..8 (exclusive). It represents one of the
    /// Base Registers (R0, R1, ..., R7).
    pub fn read(&self, base_register_index: u16) -> u16 {
        self.base_regs[base_register_index as usize]
    }

    /// Writes the given `value` to the Base Register with the given index
    ///
    /// The `base_register_index` must be in the range 0..8 (exclusive). It represents one of the
    /// Base Registers (R0, R1, ..., R7).
    pub fn write(&mut self, base_register_index: u16, value: u16) {
        self.base_regs[base_register_index as usize] = value;
    }

    /// Updates the `COND` register based on the given `last_value`
    pub fn update_cond_flags(&mut self, last_value: u16) {
        self.cond = if last_value == 0x0 {
            CondFlag::Zero
        } else if last_value >> 15 == 0x1 {
            CondFlag::Neg
        } else {
            CondFlag::Pos
        };
    }
}
