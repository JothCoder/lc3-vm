use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    /// Add
    Add,
    /// Branch
    Br,
    /// Load
    Ld,
    /// Store
    St,
    /// Jump to subroutine
    Jsr,
    /// Bitwise AND
    And,
    /// Load base + offset
    Ldr,
    /// Store base + offset
    Str,
    /// Return from interrupt (unused)
    Rti,
    /// Bitwise NOT
    Not,
    /// Load indirect
    Ldi,
    /// Store indirect
    Sti,
    /// Jump
    Jmp,
    /// Reserved (unused)
    Res,
    /// Load effective address
    Lea,
    /// System call
    Trap,
}

impl TryFrom<u16> for Opcode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use Opcode::*;

        let opcode = match value {
            0b0000 => Br,
            0b0001 => Add,
            0b0010 => Ld,
            0b0011 => St,
            0b0100 => Jsr,
            0b0101 => And,
            0b0110 => Ldr,
            0b0111 => Str,
            0b1000 => Rti,
            0b1001 => Not,
            0b1010 => Ldi,
            0b1011 => Sti,
            0b1100 => Jmp,
            0b1101 => Res,
            0b1110 => Lea,
            0b1111 => Trap,
            _ => return Err(()),
        };

        Ok(opcode)
    }
}
