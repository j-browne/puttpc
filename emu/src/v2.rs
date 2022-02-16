use crate::Machine;
use bitflags::bitflags;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Instruction {
    Nop,
    Ldav,
    Ldam,
    Sta,
    Txb,
    Add,
    Addv,
    Addm,
    Sub,
    Subv,
    Subm,
    Jmp,
    Jz,
    Jc,
    Out,
    Hlt,
}

impl FromPrimitive for Instruction {
    #[allow(clippy::cast_sign_loss)]
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u64(n as u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            x if x == Instruction::Nop as u64 => Some(Instruction::Nop),
            x if x == Instruction::Ldav as u64 => Some(Instruction::Ldav),
            x if x == Instruction::Ldam as u64 => Some(Instruction::Ldam),
            x if x == Instruction::Sta as u64 => Some(Instruction::Sta),
            x if x == Instruction::Txb as u64 => Some(Instruction::Txb),
            x if x == Instruction::Add as u64 => Some(Instruction::Add),
            x if x == Instruction::Addv as u64 => Some(Instruction::Addv),
            x if x == Instruction::Addm as u64 => Some(Instruction::Addm),
            x if x == Instruction::Sub as u64 => Some(Instruction::Sub),
            x if x == Instruction::Subv as u64 => Some(Instruction::Subv),
            x if x == Instruction::Subm as u64 => Some(Instruction::Subm),
            x if x == Instruction::Jmp as u64 => Some(Instruction::Jmp),
            x if x == Instruction::Jz as u64 => Some(Instruction::Jz),
            x if x == Instruction::Jc as u64 => Some(Instruction::Jc),
            x if x == Instruction::Out as u64 => Some(Instruction::Out),
            x if x == Instruction::Hlt as u64 => Some(Instruction::Hlt),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Register {
    Counter,
    A,
    B,
    Output,
    RamAddress,
    Instruction,
}

impl FromPrimitive for Register {
    #[allow(clippy::cast_sign_loss)]
    fn from_i64(n: i64) -> Option<Self> {
        Self::from_u64(n as u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            x if x == Register::A as u64 => Some(Register::A),
            x if x == Register::B as u64 => Some(Register::B),
            x if x == Register::Output as u64 => Some(Register::Output),
            x if x == Register::RamAddress as u64 => Some(Register::RamAddress),
            x if x == Register::Instruction as u64 => Some(Register::Instruction),
            _ => None,
        }
    }
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
    pub memory: [u8; 16],
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
            memory: [0; 16],
            controls: Controls::COUNTER_OUT | Controls::RAM_ADDR_IN,
            flags_in: Flags::ZERO,
            flags: Flags::empty(),
            micro: 0,
        }
    }

    #[must_use]
    pub fn with_input(input: &[<Self as Machine>::Input]) -> Self {
        let mut p = Self::new();
        p.set_input(input);
        p
    }

    fn buses(&self) -> (u8, Flags) {
        let mut flags = Flags::empty();

        let x = if self.controls.contains(Controls::COUNTER_OUT) {
            self.regs[Register::Counter as usize]
        } else {
            0
        } | if self.controls.contains(Controls::A_OUT) {
            self.regs[Register::A as usize]
        } else {
            0
        } | if self.controls.contains(Controls::B_OUT) {
            self.regs[Register::B as usize]
        } else {
            0
        } | if self.controls.contains(Controls::INSTRUCTION_OUT) {
            self.regs[Register::Instruction as usize] & 0xF
        } else {
            0
        } | if self.controls.contains(Controls::RAM_OUT) {
            self.memory[self.regs[Register::RamAddress as usize] as usize]
        } else {
            0
        } | if self.controls.contains(Controls::ADDER_OUT) {
            let (res, ov) = if self.controls.contains(Controls::SUBTRACT) {
                self.regs[Register::A as usize].overflowing_sub(self.regs[Register::B as usize])
            } else {
                self.regs[Register::A as usize].overflowing_add(self.regs[Register::B as usize])
            };

            if res == 0 {
                flags |= Flags::ZERO;
            }
            if ov {
                flags |= Flags::CARRY;
            }

            res
        } else {
            0
        };

        (x, flags)
    }

    #[allow(clippy::match_same_arms)]
    fn controls(&self) -> Controls {
        let instr = Instruction::from_u8(self.regs[Register::Instruction as usize] >> 4)
            .expect("a u8 right shifted 4 is a valid instruction");
        match (instr, self.micro) {
            (_, 0) => Controls::COUNTER_OUT | Controls::RAM_ADDR_IN,
            (_, 1) => Controls::COUNTER_INCREMENT | Controls::RAM_OUT | Controls::INSTRUCTION_IN,
            (Instruction::Nop, 2) => Controls::RESET_MICRO,
            (Instruction::Ldav, 2) => {
                Controls::INSTRUCTION_OUT | Controls::A_IN | Controls::RESET_MICRO
            }
            (Instruction::Ldam, 2) => Controls::INSTRUCTION_OUT | Controls::RAM_ADDR_IN,
            (Instruction::Ldam, 3) => Controls::RAM_OUT | Controls::A_IN | Controls::RESET_MICRO,
            (Instruction::Sta, 2) => Controls::INSTRUCTION_OUT | Controls::RAM_ADDR_IN,
            (Instruction::Sta, 3) => Controls::A_OUT | Controls::RAM_IN | Controls::RESET_MICRO,
            (Instruction::Txb, 2) => Controls::A_OUT | Controls::B_IN | Controls::RESET_MICRO,
            (Instruction::Add, 2) => {
                Controls::ADDER_OUT | Controls::A_IN | Controls::FLAGS_IN | Controls::RESET_MICRO
            }
            (Instruction::Addv, 2) => Controls::INSTRUCTION_OUT | Controls::B_IN,
            (Instruction::Addv, 3) => {
                Controls::ADDER_OUT | Controls::A_IN | Controls::FLAGS_IN | Controls::RESET_MICRO
            }
            (Instruction::Addm, 2) => Controls::INSTRUCTION_OUT | Controls::RAM_ADDR_IN,
            (Instruction::Addm, 3) => Controls::RAM_OUT | Controls::B_IN | Controls::RESET_MICRO,
            (Instruction::Addm, 4) => {
                Controls::ADDER_OUT | Controls::A_IN | Controls::FLAGS_IN | Controls::RESET_MICRO
            }
            (Instruction::Sub, 2) => {
                Controls::ADDER_OUT
                    | Controls::A_IN
                    | Controls::FLAGS_IN
                    | Controls::RESET_MICRO
                    | Controls::SUBTRACT
            }
            (Instruction::Subv, 2) => {
                Controls::INSTRUCTION_OUT | Controls::B_IN | Controls::SUBTRACT
            }
            (Instruction::Subv, 3) => {
                Controls::ADDER_OUT
                    | Controls::A_IN
                    | Controls::FLAGS_IN
                    | Controls::RESET_MICRO
                    | Controls::SUBTRACT
            }
            (Instruction::Subm, 2) => {
                Controls::INSTRUCTION_OUT | Controls::RAM_ADDR_IN | Controls::SUBTRACT
            }
            (Instruction::Subm, 3) => {
                Controls::RAM_OUT | Controls::B_IN | Controls::RESET_MICRO | Controls::SUBTRACT
            }
            (Instruction::Subm, 4) => {
                Controls::ADDER_OUT
                    | Controls::A_IN
                    | Controls::FLAGS_IN
                    | Controls::RESET_MICRO
                    | Controls::SUBTRACT
            }
            (Instruction::Jmp, 2) => {
                Controls::INSTRUCTION_OUT | Controls::JUMP | Controls::RESET_MICRO
            }
            (Instruction::Jz, 2) => {
                Controls::INSTRUCTION_OUT | Controls::JUMP_IF_ZERO | Controls::RESET_MICRO
            }
            (Instruction::Jc, 2) => {
                Controls::INSTRUCTION_OUT | Controls::JUMP_IF_CARRY | Controls::RESET_MICRO
            }
            (Instruction::Out, 2) => Controls::A_OUT | Controls::OUTPUT_IN | Controls::RESET_MICRO,
            (Instruction::Hlt, 2) => Controls::HALT | Controls::RESET_MICRO,
            (_, _) => Controls::empty(),
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
        self.controls.contains(Controls::HALT)
    }

    fn set_input(&mut self, input: &[Self::Input]) {
        let len = input.len();
        assert!(len <= 16);
        let memory = &mut self.memory[..len];
        memory.copy_from_slice(input);
    }

    fn step(&mut self) -> Option<Self::Output> {
        let mut out = None;
        let (data, flags) = self.buses();

        if self.controls.contains(Controls::RAM_ADDR_IN) {
            self.regs[Register::RamAddress as usize] = data;
        }
        if self.controls.contains(Controls::OUTPUT_IN) {
            self.regs[Register::Output as usize] = data;
            out = Some(data);
        }
        if self.controls.contains(Controls::A_IN) {
            self.regs[Register::A as usize] = data;
        }
        if self.controls.contains(Controls::B_IN) {
            self.regs[Register::B as usize] = data;
        }
        if self.controls.contains(Controls::INSTRUCTION_IN) {
            self.regs[Register::Instruction as usize] = data;
        }
        if self.controls.contains(Controls::RAM_IN) {
            self.memory[self.regs[Register::RamAddress as usize] as usize] = data;
        }

        self.flags_in = flags;
        if self.controls.contains(Controls::FLAGS_IN) {
            self.flags = self.flags_in;
        }

        if self.controls.contains(Controls::JUMP)
            || (self.controls.contains(Controls::JUMP_IF_ZERO) && self.flags.contains(Flags::ZERO))
            || (self.controls.contains(Controls::JUMP_IF_CARRY)
                && self.flags.contains(Flags::CARRY))
        {
            self.regs[Register::Counter as usize] = data & 0xF;
        }
        if self.controls.contains(Controls::COUNTER_INCREMENT) {
            self.regs[Register::Counter as usize] += 1;
        }

        self.controls = self.controls();

        self.micro += 1;
        if self.controls.contains(Controls::RESET_MICRO) || self.micro > 4 {
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
