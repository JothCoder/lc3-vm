//! All instructions that are supported and have an implementation
//!
//! Instructions are 16-bit values and have a specific binary encoding. The first four bits of
//! each instruction express the [`Opcode`](super::Opcode).

mod trap;

use super::{utils::bit_ops::sign_extend, Memory, Registers};
use trap::TrapCode;

use std::convert::TryFrom;

/// Parses and performs the `BR` (*branch*) instruction
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   0   0   0 │ n │ z │ p │             PCoffset9             │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly formats
///
/// ```asm
/// BR    LABEL
/// BRn   LABEL
/// BRz   LABEL
/// BRp   LABEL
/// BRzp  LABEL
/// BRnp  LABEL
/// BRnz  LABEL
/// BRnzp LABEL
/// ```
pub fn br(instr: u16, regs: &mut Registers) {
    // Condition flags (Negative, Zero, Positive)
    // Not masked because the bitwise AND with `regs.cond` acts like a mask.
    let nzp = instr >> 9;
    if (nzp & (regs.cond as u16)) > 0 {
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        regs.pc = regs.pc.wrapping_add(pc_offset);
    }
}

/// Parses and performs the `ADD` (*addition*) instruction
///
/// **Note**: this instruction updates the `COND` register (NZP flags) based on the value written
/// to `DR`.
///
/// # Binary encodings
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   0   0   1 │     DR    │    SR1    │ 0 │ 0   0 │    SR2    │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
///
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   0   0   1 │     DR    │    SR1    │ 1 │        imm5       │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly formats
///
/// ```asm
/// ADD  DR, SR1, SR2
/// ADD  DR, SR1, imm5
/// ```
pub fn add(instr: u16, regs: &mut Registers) {
    let dest_reg = (instr >> 9) & 0x7;
    let src_reg1 = (instr >> 6) & 0x7;
    let mode = (instr >> 5) & 0x1;
    let value;
    match mode {
        // Immediate mode
        0x1 => {
            let imm = sign_extend(instr & 0x1F, 5);
            value = regs.read(src_reg1).wrapping_add(imm);
        }
        // Register mode
        0x0 => {
            let src_reg2 = instr & 0x7;
            value = regs.read(src_reg1).wrapping_add(regs.read(src_reg2));
        }
        _ => unreachable!(),
    }

    regs.write(dest_reg, value as u16);
    regs.update_cond_flags(value);
}

/// Parses and performs the `LD` (*load*) instruction
///
/// **Note**: this instruction updates the `COND` register (NZP flags) based on the value written
/// to `DR`.
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   0   1   0 │     DR    │             PCoffset9             │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// LD   DR, LABEL
/// ```
pub fn ld(instr: u16, regs: &mut Registers, mem: &mut Memory) {
    let dest_reg = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let value = mem.read(regs.pc.wrapping_add(pc_offset));
    regs.write(dest_reg, value);
    regs.update_cond_flags(value);
}

/// Parses and performs the `ST` (*store*) instruction
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   0   1   1 │     SR    │             PCoffset9             │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// ST   SR, LABEL
/// ```
pub fn st(instr: u16, regs: &Registers, mem: &mut Memory) {
    let src_reg = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let value = regs.read(src_reg);
    mem.write(regs.pc.wrapping_add(pc_offset), value);
}

/// Parses and performs the `JSR` (*jump to subroutine*) instruction
///
/// # Binary encodings
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   1   0   0 │ 1 │                 PCoffset11                │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
///
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   1   0   0 │ 0 │ 0   0 │   BaseR   │ 0   0   0   0   0   0 │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly formats
///
/// ```asm
/// JSR  LABEL
/// JSRR BaseR
/// ```
pub fn jsr(instr: u16, regs: &mut Registers) {
    regs.write(7, regs.pc);
    let flag = (instr >> 11) & 0x1;
    match flag {
        // JSR
        0x1 => {
            let pc_offset = sign_extend(instr & 0x7FF, 11);
            regs.pc = regs.pc.wrapping_add(pc_offset);
        }
        // JSRR
        0x0 => {
            let base_reg = (instr >> 6) & 0x7;
            regs.pc = regs.read(base_reg);
        }
        _ => unreachable!(),
    }
}

/// Parses and performs the `AND` (*bitwise AND*) instruction
///
/// **Note**: this instruction updates the `COND` register (NZP flags) based on the value written
/// to `DR`.
///
/// # Binary encodings
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   1   0   1 │     DR    │    SR1    │ 0 │ 0   0 │    SR2    │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
///
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   1   0   1 │     DR    │    SR1    │ 1 │        imm5       │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly formats
///
/// ```asm
/// AND  DR, SR1, SR2
/// AND  DR, SR1, imm5
/// ```
pub fn and(instr: u16, regs: &mut Registers) {
    let dest_reg = (instr >> 9) & 0x7;
    let src_reg1 = (instr >> 6) & 0x7;
    let mode = (instr >> 5) & 0x1;
    let value;
    match mode {
        // Immediate mode
        0x1 => {
            let imm = sign_extend(instr & 0x1F, 5);
            value = regs.read(src_reg1) & imm;
        }
        // Register mode
        0x0 => {
            let src_reg2 = instr & 0x7;
            value = regs.read(src_reg1) & regs.read(src_reg2);
        }
        _ => unreachable!(),
    }

    regs.write(dest_reg, value);
    regs.update_cond_flags(value);
}

/// Parses and performs the `LDR` (*load base + offset*) instruction
///
/// **Note**: this instruction updates the `COND` register (NZP flags) based on the value written
/// to `DR`.
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   1   1   0 │     DR    │   BaseR   │        offset6        │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// LDR  DR, BaseR, offset6
/// ```
pub fn ldr(instr: u16, regs: &mut Registers, mem: &mut Memory) {
    let dest_reg = (instr >> 9) & 0x7;
    let base_reg = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let value = mem.read(regs.read(base_reg).wrapping_add(offset));
    regs.write(dest_reg, value);
    regs.update_cond_flags(value);
}

/// Parses and performs the `STR` (*store base + offset*) instruction
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 0   1   1   1 │     SR    │   BaseR   │        offset6        │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// STR  SR, BaseR, offset6
/// ```
pub fn str(instr: u16, regs: &Registers, mem: &mut Memory) {
    let src_reg = (instr >> 9) & 0x7;
    let base_reg = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let value = regs.read(src_reg);
    mem.write(regs.read(base_reg).wrapping_add(offset), value);
}

/// Parses and performs the `NOT` (*bitwise complement*) instruction
///
/// **Note**: this instruction updates the `COND` register (NZP flags) based on the value written
/// to `DR`.
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 1   0   0   1 │     DR    │     SR    │ 1 │ 1   1   1   1   1 │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// NOT  DR, SR
/// ```
pub fn not(instr: u16, regs: &mut Registers) {
    let dest_reg = (instr >> 9) & 0x7;
    let src_reg = (instr >> 6) & 0x7;
    let value = !regs.read(src_reg);
    regs.write(dest_reg, value);
    regs.update_cond_flags(value);
}

/// Parses and performs the `LDI` (*load indirect*) instruction
///
/// **Note**: this instruction updates the `COND` register (NZP flags) based on the value written
/// to `DR`.
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 1   0   1   0 │     DR    │             PCoffset9             │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// LDI  DR, LABEL
/// ```
pub fn ldi(instr: u16, regs: &mut Registers, mem: &mut Memory) {
    let dest_reg = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let mem_addr = mem.read(regs.pc.wrapping_add(pc_offset));
    let value = mem.read(mem_addr);
    regs.write(dest_reg, value);
    regs.update_cond_flags(value);
}

/// Parses and performs the `STI` (*store indirect*) instruction
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 1   0   1   1 │     SR    │             PCoffset9             │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// STI  SR, LABEL
/// ```
pub fn sti(instr: u16, regs: &Registers, mem: &mut Memory) {
    let src_reg = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let mem_addr = mem.read(regs.pc.wrapping_add(pc_offset));
    mem.write(mem_addr, regs.read(src_reg));
}

/// Parses and performs the `JMP` (*jump*) instruction
///
/// Note that if the instruction's BaseR is R7, this instruction is equivalent to the `RET`
/// (*return from subroutine*) instruction.
///
/// # Binary encodings
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 1   1   0   0 │ 0   0   0 │   BaseR   │ 0   0   0   0   0   0 │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
///
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 1   1   0   0 │ 0   0   0 │ 1   1   1 │ 0   0   0   0   0   0 │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly formats
///
/// ```asm
/// JMP  BaseR
/// RET
/// ```
pub fn jmp(instr: u16, regs: &mut Registers) {
    let base_reg = (instr >> 6) & 0x7;
    regs.pc = regs.read(base_reg);
}

/// Parses and performs the `LEA` (*load effective address*) instruction
///
/// **Note**: this instruction updates the `COND` register (NZP flags) based on the value written
/// to `DR`.
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 1   0   1   0 │     DR    │             PCoffset9             │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// LDI  DR, LABEL
/// ```
pub fn lea(instr: u16, regs: &mut Registers) {
    let dest_reg = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let value = regs.pc.wrapping_add(pc_offset);
    regs.write(dest_reg, value);
    regs.update_cond_flags(value);
}

/// Parses and performs the `TRAP` (*system call*) instruction; returns whether the vm should halt
///
/// # Binary encoding
///
/// ```plain
/// ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
/// │ 1   1   1   1 │ 0   0   0   0 │           trapvect8           │
/// └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
/// ```
///
/// # Assembly format
///
/// ```asm
/// TRAP trapvector8
/// ```
pub fn trap(instr: u16, regs: &mut Registers, mem: &mut Memory) -> bool {
    let trapvector = instr & 0xFF;
    let trap_code = TrapCode::try_from(trapvector).unwrap_or_else(|_| panic!("Unsupported trap code: {:#010b}", trapvector));
    match trap_code {
        TrapCode::Getc => trap::getc(regs),
        TrapCode::Out => trap::out(regs),
        TrapCode::Puts => trap::puts(regs, mem),
        TrapCode::In => trap::r#in(regs),
        TrapCode::Putsp => trap::putsp(regs, mem),
        TrapCode::Halt => {
            trap::halt();
            return true;
        }
    }
    false
}
