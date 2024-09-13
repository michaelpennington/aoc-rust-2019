use std::str::FromStr;

use anyhow::anyhow;

use crate::util::digits::DigitsIter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    program: Vec<i32>,
    pc: usize,
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut program = Vec::with_capacity(s.split(',').count());
        for n in s.trim().split(',') {
            program.push(n.parse()?);
        }
        Ok(Self { program, pc: 0 })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Add,
    Mul,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl TryFrom<i32> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            99 => Ok(Self::Halt),
            _ => Err(anyhow!("{value} is not a valid opcode")),
        }
    }
}

impl TryFrom<i32> for ParameterMode {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(anyhow!("{value} is not a valid parameter mode")),
        }
    }
}

impl Program {
    pub fn set(&mut self, index: usize, val: i32) {
        self.program[index] = val;
    }

    pub fn get(&self, index: usize) -> i32 {
        self.program[index]
    }

    pub fn execute(&mut self) {
        self.execute_with_input(std::iter::empty());
    }

    pub fn execute_with_input(&mut self, input: impl IntoIterator<Item = i32>) -> i32 {
        let mut last_seen = 0;
        let mut input_iter = input.into_iter();
        loop {
            match Self::parse_opcode(self.program[self.pc]) {
                (Instruction::Add, pmodes) => {
                    let x = self.get_with_mode(self.pc + 1, pmodes[2]);
                    let y = self.get_with_mode(self.pc + 2, pmodes[1]);
                    let addr = self.program[self.pc + 3];
                    self.program[addr as usize] = x + y;
                    self.pc += 4;
                }
                (Instruction::Mul, pmodes) => {
                    let x = self.get_with_mode(self.pc + 1, pmodes[2]);
                    let y = self.get_with_mode(self.pc + 2, pmodes[1]);
                    let addr = self.program[self.pc + 3];
                    self.program[addr as usize] = x * y;
                    self.pc += 4;
                }
                (Instruction::Input, _) => {
                    let addr = self.program[self.pc + 1] as usize;
                    self.program[addr] = input_iter.next().unwrap();
                    self.pc += 2;
                }
                (Instruction::Output, pmodes) => {
                    let val = self.get_with_mode(self.pc + 1, pmodes[2]);
                    last_seen = val;
                    self.pc += 2;
                    // break val;
                }
                (Instruction::JumpIfTrue, pmodes) => {
                    if self.get_with_mode(self.pc + 1, pmodes[2]) != 0 {
                        self.pc = self.get_with_mode(self.pc + 2, pmodes[1]) as usize
                    } else {
                        self.pc += 3;
                    }
                }
                (Instruction::JumpIfFalse, pmodes) => {
                    if self.get_with_mode(self.pc + 1, pmodes[2]) == 0 {
                        self.pc = self.get_with_mode(self.pc + 2, pmodes[1]) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                (Instruction::LessThan, pmodes) => {
                    let addr = self.program[self.pc + 3] as usize;
                    if self.get_with_mode(self.pc + 1, pmodes[2])
                        < self.get_with_mode(self.pc + 2, pmodes[1])
                    {
                        self.program[addr] = 1;
                    } else {
                        self.program[addr] = 0;
                    }
                    self.pc += 4;
                }
                (Instruction::Equals, pmodes) => {
                    let addr = self.program[self.pc + 3] as usize;
                    if self.get_with_mode(self.pc + 1, pmodes[2])
                        == self.get_with_mode(self.pc + 2, pmodes[1])
                    {
                        self.program[addr] = 1;
                    } else {
                        self.program[addr] = 0;
                    }
                    self.pc += 4;
                }
                (Instruction::Halt, _) => {
                    break last_seen;
                }
            }
        }
    }

    pub fn execute_with_input_to_vec(
        &mut self,
        input: impl IntoIterator<Item = i32>,
    ) -> (Option<Vec<i32>>, Option<i32>) {
        let mut output = Vec::new();
        let mut input_iter = input.into_iter();
        loop {
            match Self::parse_opcode(self.program[self.pc]) {
                (Instruction::Add, pmodes) => {
                    let x = self.get_with_mode(self.pc + 1, pmodes[2]);
                    let y = self.get_with_mode(self.pc + 2, pmodes[1]);
                    let addr = self.program[self.pc + 3];
                    self.program[addr as usize] = x + y;
                    self.pc += 4;
                }
                (Instruction::Mul, pmodes) => {
                    let x = self.get_with_mode(self.pc + 1, pmodes[2]);
                    let y = self.get_with_mode(self.pc + 2, pmodes[1]);
                    let addr = self.program[self.pc + 3];
                    self.program[addr as usize] = x * y;
                    self.pc += 4;
                }
                (Instruction::Input, _) => {
                    let addr = self.program[self.pc + 1] as usize;
                    if let Some(i) = input_iter.next() {
                        self.program[addr] = i;
                        self.pc += 2;
                    } else {
                        break (Some(output), None);
                    }
                }
                (Instruction::Output, pmodes) => {
                    let val = self.get_with_mode(self.pc + 1, pmodes[2]);
                    output.push(val);
                    self.pc += 2;
                }
                (Instruction::JumpIfTrue, pmodes) => {
                    if self.get_with_mode(self.pc + 1, pmodes[2]) != 0 {
                        self.pc = self.get_with_mode(self.pc + 2, pmodes[1]) as usize
                    } else {
                        self.pc += 3;
                    }
                }
                (Instruction::JumpIfFalse, pmodes) => {
                    if self.get_with_mode(self.pc + 1, pmodes[2]) == 0 {
                        self.pc = self.get_with_mode(self.pc + 2, pmodes[1]) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                (Instruction::LessThan, pmodes) => {
                    let addr = self.program[self.pc + 3] as usize;
                    if self.get_with_mode(self.pc + 1, pmodes[2])
                        < self.get_with_mode(self.pc + 2, pmodes[1])
                    {
                        self.program[addr] = 1;
                    } else {
                        self.program[addr] = 0;
                    }
                    self.pc += 4;
                }
                (Instruction::Equals, pmodes) => {
                    let addr = self.program[self.pc + 3] as usize;
                    if self.get_with_mode(self.pc + 1, pmodes[2])
                        == self.get_with_mode(self.pc + 2, pmodes[1])
                    {
                        self.program[addr] = 1;
                    } else {
                        self.program[addr] = 0;
                    }
                    self.pc += 4;
                }
                (Instruction::Halt, _) => {
                    break (None, Some(*output.last().unwrap()));
                }
            }
        }
    }

    fn get_with_mode(&self, i: usize, pmode: ParameterMode) -> i32 {
        match pmode {
            ParameterMode::Position => self.program[self.program[i] as usize],
            ParameterMode::Immediate => self.program[i],
        }
    }

    fn parse_opcode(code: i32) -> (Instruction, [ParameterMode; 3]) {
        let opcode = code % 100;
        let mut rest = code / 100;
        rest += 1000;
        let mut digis = DigitsIter::new(rest).skip(1).take(3);
        let (one, two, three) = (
            digis.next().unwrap(),
            digis.next().unwrap(),
            digis.next().unwrap(),
        );
        let i = opcode.try_into().unwrap();
        let p_modes = [
            one.try_into().unwrap(),
            two.try_into().unwrap(),
            three.try_into().unwrap(),
        ];
        (i, p_modes)
    }
}
