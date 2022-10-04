use crate::Machine;
use bitflags::bitflags;
use derive_try_from_primitive::TryFromPrimitive;
use std::{convert::TryFrom, fmt};

use Controls as C;
use Flags as F;
use Instruction as I;
use Register as R;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum Instruction {
    Nop = 0x00,
    MovAB = 0x01,
    MovAV = 0x02,
    MovAM = 0x03,
    MovBA = 0x04,
    MovBV = 0x05,
    MovBM = 0x06,
    MovMA = 0x07,
    MovMB = 0x08,
    MovMV = 0x09,
    MovMM = 0x0a,
    AddAB = 0x10,
    AddAV = 0x11,
    AddAM = 0x12,
    AddVB = 0x13,
    AddVV = 0x14,
    AddVM = 0x15,
    AddMB = 0x16,
    AddMV = 0x17,
    AddMM = 0x18,
    SubAB = 0x20,
    SubAV = 0x21,
    SubAM = 0x22,
    SubVB = 0x23,
    SubVV = 0x24,
    SubVM = 0x25,
    SubMB = 0x26,
    SubMV = 0x27,
    SubMM = 0x28,
    Jmp = 0xD0,
    Jz = 0xD1,
    Jnz = 0xD2,
    Jc = 0xD3,
    Jnc = 0xD4,
    Out = 0xE0,
    Hlt = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum Register {
    Counter,
    A,
    B,
    Output,
    RamAddress,
    Instruction,
}

bitflags! {
    #[derive(Default)]
    pub struct Controls: u32 {
        const CLOCK = 0b0000_0000_0000_0000_0000_0001;
        const RESET = 0b0000_0000_0000_0000_0000_0010;
        const RESET_RAM = 0b0000_0000_0000_0000_0000_0100;
        const HALT = 0b0000_0000_0000_0000_0000_1000;
        const SUBTRACT = 0b0000_0000_0000_0000_0001_0000;
        const ADDER_OUT = 0b0000_0000_0000_0000_0010_0000;
        const RAM_ADDR_IN = 0b0000_0000_0000_0000_0100_0000;
        const OUTPUT_IN = 0b0000_0000_0000_0000_1000_0000;
        const A_IN = 0b0000_0000_0000_0001_0000_0000;
        const A_OUT = 0b0000_0000_0000_0010_0000_0000;
        const B_IN = 0b0000_0000_0000_0100_0000_0000;
        const B_OUT = 0b0000_0000_0000_1000_0000_0000;
        const INSTRUCTION_IN = 0b0000_0000_0001_0000_0000_0000;
        const INSTRUCTION_OUT = 0b0000_0000_0010_0000_0000_0000;
        const RAM_IN = 0b0000_0000_0100_0000_0000_0000;
        const RAM_OUT = 0b0000_0000_1000_0000_0000_0000;
        const COUNTER_OUT = 0b0000_0001_0000_0000_0000_0000;
        const COUNTER_INCREMENT = 0b0000_0010_0000_0000_0000_0000;
        const JUMP = 0b0000_0100_0000_0000_0000_0000;
        const JUMP_IF_ZERO = 0b0000_1000_0000_0000_0000_0000;
        const JUMP_IF_CARRY = 0b0001_0000_0000_0000_0000_0000;
        const FLAGS_IN = 0b0010_0000_0000_0000_0000_0000;
        const RESET_MICRO = 0b0100_0000_0000_0000_0000_0000;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Flags:u32 {
        const ZERO = 0b01;
        const CARRY = 0b10;
    }
}

#[derive(Debug)]
pub struct PuttPc {
    pub regs: [u8; 6],
    pub memory: [u8; 256],
    pub controls: Controls,
    pub flags_in: Flags,
    pub flags: Flags,
    pub micro: usize,
}

impl PuttPc {
    #[must_use]
    pub fn new() -> Self {
        PuttPc {
            regs: [0; 6],
            memory: [0; 256],
            controls: C::COUNTER_OUT | C::RAM_ADDR_IN,
            flags_in: F::ZERO,
            flags: F::empty(),
            micro: 0,
        }
    }

    #[must_use]
    pub fn with_input(input: &[<Self as Machine>::Input]) -> Self {
        let mut p = Self::new();
        p.set_input(input);
        p
    }

    fn data_bus(&self) -> u8 {
        let mut data = 0;
        if self.controls.contains(C::COUNTER_OUT) {
            data |= self.regs[R::Counter as usize];
        }
        if self.controls.contains(C::A_OUT) {
            data |= self.regs[R::A as usize];
        }
        if self.controls.contains(C::B_OUT) {
            data |= self.regs[R::B as usize];
        }
        if self.controls.contains(C::INSTRUCTION_OUT) {
            data |= self.regs[R::Instruction as usize] & 0xF;
        }
        if self.controls.contains(C::RAM_OUT) {
            data |= self.memory[self.regs[R::RamAddress as usize] as usize];
        }
        if self.controls.contains(C::ADDER_OUT) {
            let (adder_sum, _) = if self.controls.contains(C::SUBTRACT) {
                self.regs[R::A as usize].overflowing_sub(self.regs[R::B as usize])
            } else {
                self.regs[R::A as usize].overflowing_add(self.regs[R::B as usize])
            };

            data |= adder_sum;
        };

        data
    }

    fn flags_in_bus(&self) -> Flags {
        let (adder_sum, adder_overflow) = if self.controls.contains(C::SUBTRACT) {
            self.regs[R::A as usize].overflowing_sub(self.regs[R::B as usize])
        } else {
            self.regs[R::A as usize].overflowing_add(self.regs[R::B as usize])
        };

        let mut flags_in = F::empty();
        if adder_sum == 0 {
            flags_in |= F::ZERO;
        }
        if adder_overflow {
            flags_in |= F::CARRY;
        }

        flags_in
    }

    fn controls_bus(&self) -> Controls {
        let instr = I::try_from(self.regs[R::Instruction as usize]).unwrap();
        match (instr, self.micro) {
            (_, 0) => C::COUNTER_OUT | C::RAM_ADDR_IN,
            (_, 1) => C::COUNTER_INCREMENT | C::RAM_OUT | C::INSTRUCTION_IN,
            (I::Nop, 2) => C::RESET_MICRO,
            (I::MovAB, _) => todo!(),
            (I::MovAV, _) => todo!(),
            (I::MovAM, _) => todo!(),
            (I::MovBA, _) => todo!(),
            (I::MovBV, _) => todo!(),
            (I::MovBM, _) => todo!(),
            (I::MovMA, _) => todo!(),
            (I::MovMB, _) => todo!(),
            (I::MovMV, _) => todo!(),
            (I::MovMM, _) => todo!(),
            (I::AddAB, _) => todo!(),
            (I::AddAV, _) => todo!(),
            (I::AddAM, _) => todo!(),
            (I::AddVB, _) => todo!(),
            (I::AddVV, _) => todo!(),
            (I::AddVM, _) => todo!(),
            (I::AddMB, _) => todo!(),
            (I::AddMV, _) => todo!(),
            (I::AddMM, _) => todo!(),
            (I::SubAB, _) => todo!(),
            (I::SubAV, _) => todo!(),
            (I::SubAM, _) => todo!(),
            (I::SubVB, _) => todo!(),
            (I::SubVV, _) => todo!(),
            (I::SubVM, _) => todo!(),
            (I::SubMB, _) => todo!(),
            (I::SubMV, _) => todo!(),
            (I::SubMM, _) => todo!(),
            (I::Jmp, _) => todo!(),
            (I::Jz, _) => todo!(),
            (I::Jnz, _) => todo!(),
            (I::Jc, _) => todo!(),
            (I::Jnc, _) => todo!(),
            (I::Out, 2) => C::A_OUT | C::OUTPUT_IN | C::RESET_MICRO,
            (I::Hlt, 2) => C::HALT | C::RESET_MICRO,
            (_, _) => C::empty(),
        }
    }
}

impl Default for PuttPc {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine for PuttPc {
    type Input = u8;
    type Output = u8;

    fn is_halted(&self) -> bool {
        self.controls.contains(C::HALT)
    }

    fn set_input(&mut self, input: &[Self::Input]) {
        let len = input.len();
        assert!(len <= 16);
        let memory = &mut self.memory[..len];
        memory.copy_from_slice(input);
    }

    fn step(&mut self) -> Option<Self::Output> {
        let mut out = None;
        let data = self.data_bus();

        if self.controls.contains(C::RAM_ADDR_IN) {
            self.regs[R::RamAddress as usize] = data;
        }
        if self.controls.contains(C::OUTPUT_IN) {
            self.regs[R::Output as usize] = data;
            out = Some(data);
        }
        if self.controls.contains(C::A_IN) {
            self.regs[R::A as usize] = data;
        }
        if self.controls.contains(C::B_IN) {
            self.regs[R::B as usize] = data;
        }
        if self.controls.contains(C::INSTRUCTION_IN) {
            self.regs[R::Instruction as usize] = data;
        }
        if self.controls.contains(C::RAM_IN) {
            self.memory[self.regs[R::RamAddress as usize] as usize] = data;
        }

        // Set the flags register and then calculate the NEXT step's flags_in
        if self.controls.contains(C::FLAGS_IN) {
            self.flags = self.flags_in;
        }
        self.flags_in = self.flags_in_bus();

        if self.controls.contains(C::JUMP)
            || (self.controls.contains(C::JUMP_IF_ZERO) && self.flags.contains(F::ZERO))
            || (self.controls.contains(C::JUMP_IF_CARRY) && self.flags.contains(F::CARRY))
        {
            self.regs[R::Counter as usize] = data & 0xF;
        }
        if self.controls.contains(C::COUNTER_INCREMENT) {
            self.regs[R::Counter as usize] += 1;
        }

        self.controls = self.controls_bus();

        self.micro += 1;
        if self.controls.contains(C::RESET_MICRO) || self.micro > 4 {
            self.micro = 0;
        }

        out
    }

    fn run(&mut self) -> Vec<Self::Output> {
        let mut output = Vec::new();
        while !self.is_halted() {
            let out = self.step();
            output.extend(out);
        }
        output
    }
}

impl IntoIterator for PuttPc {
    type Item = u8;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl fmt::Display for PuttPc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Registers")?;
        for (i, v) in self.regs.iter().enumerate() {
            let reg = u8::try_from(i).unwrap();
            let reg = R::try_from(reg).unwrap();
            writeln!(f, "  {:<11} {v:>3} ({v:08b})", format!("{:?}", reg))?;
        }

        writeln!(f, "Memory")?;
        for (i, v) in self.memory.iter().enumerate() {
            writeln!(f, "  {i:<2} {v:>3} ({v:08b})")?;
        }

        writeln!(f, "Controls")?;
        writeln!(f, "  {:?}", self.controls)?;

        writeln!(f, "Flags")?;
        writeln!(f, "  {:?}", self.flags)?;

        writeln!(f, "Micro")?;
        writeln!(f, "  {} ({:04b})", self.micro, self.micro)?;

        Ok(())
    }
}

pub struct IntoIter(PuttPc);

impl Iterator for IntoIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.0.is_halted() {
                break None;
            }
            if let Some(out) = self.0.step() {
                break Some(out);
            }
        }
    }
}
