use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign},
    str::FromStr,
};

use anyhow::anyhow;

use crate::util::digits::DigitsIter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program<T> {
    program: Vec<T>,
    pc: usize,
}

impl<T> FromStr for Program<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
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

impl TryFrom<usize> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
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

impl TryFrom<usize> for ParameterMode {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(anyhow!("{value} is not a valid parameter mode")),
        }
    }
}

impl<T> Program<T>
where
    T: TryInto<usize>
        + TryFrom<usize>
        + Clone
        + Copy
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + AddAssign
        + PartialOrd
        + RemAssign
        + MulAssign
        + DivAssign
        + Display,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
    for<'a> &'a T: Rem<T, Output = T>,
{
    pub fn set(&mut self, index: usize, val: T) {
        self.program[index] = val;
    }

    pub fn get(&self, index: usize) -> T {
        self.program[index]
    }

    pub fn execute(&mut self) {
        self.execute_with_input(std::iter::empty());
    }

    pub fn execute_with_input(&mut self, mut input: impl Iterator<Item = T>) -> T {
        let mut last_seen = 0.try_into().unwrap();
        loop {
            match Self::parse_opcode(self.program[self.pc]) {
                (Instruction::Add, pmodes) => {
                    let x = self.get_with_mode(self.pc + 1, pmodes[2]);
                    let y = self.get_with_mode(self.pc + 2, pmodes[1]);
                    let addr = self.program[self.pc + 3];
                    self.program[addr.try_into().unwrap()] = x + y;
                    self.pc += 4;
                }
                (Instruction::Mul, pmodes) => {
                    let x = self.get_with_mode(self.pc + 1, pmodes[2]);
                    let y = self.get_with_mode(self.pc + 2, pmodes[1]);
                    let addr = self.program[self.pc + 3];
                    self.program[addr.try_into().unwrap()] = x * y;
                    self.pc += 4;
                }
                (Instruction::Input, _) => {
                    let addr = self.program[self.pc + 1].try_into().unwrap();
                    self.program[addr] = input.next().unwrap();
                    self.pc += 2;
                }
                (Instruction::Output, pmodes) => {
                    let val = self.get_with_mode(self.pc + 1, pmodes[2]);
                    last_seen = val;
                    self.pc += 2;
                }
                (Instruction::JumpIfTrue, pmodes) => {
                    if self.get_with_mode(self.pc + 1, pmodes[2]) != 0.try_into().unwrap() {
                        self.pc = self
                            .get_with_mode(self.pc + 2, pmodes[1])
                            .try_into()
                            .unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                (Instruction::JumpIfFalse, pmodes) => {
                    if self.get_with_mode(self.pc + 1, pmodes[2]) == 0.try_into().unwrap() {
                        self.pc = self
                            .get_with_mode(self.pc + 2, pmodes[1])
                            .try_into()
                            .unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                (Instruction::LessThan, pmodes) => {
                    let addr = self.program[self.pc + 3].try_into().unwrap();
                    if self.get_with_mode(self.pc + 1, pmodes[2])
                        < self.get_with_mode(self.pc + 2, pmodes[1])
                    {
                        self.program[addr] = 1.try_into().unwrap();
                    } else {
                        self.program[addr] = 0.try_into().unwrap();
                    }
                    self.pc += 4;
                }
                (Instruction::Equals, pmodes) => {
                    let addr = self.program[self.pc + 3].try_into().unwrap();
                    if self.get_with_mode(self.pc + 1, pmodes[2])
                        == self.get_with_mode(self.pc + 2, pmodes[1])
                    {
                        self.program[addr] = 1.try_into().unwrap();
                    } else {
                        self.program[addr] = 0.try_into().unwrap();
                    }
                    self.pc += 4;
                }
                (Instruction::Halt, _) => {
                    break last_seen;
                }
            }
        }
    }

    fn get_with_mode(&self, i: usize, pmode: ParameterMode) -> T {
        match pmode {
            ParameterMode::Position => self.program[self.program[i].try_into().unwrap()],
            ParameterMode::Immediate => self.program[i],
        }
    }

    fn parse_opcode(code: T) -> (Instruction, [ParameterMode; 3]) {
        let opcode = mod100(&code);
        let mut rest = code / 100.try_into().unwrap();
        rest += 1000.try_into().unwrap();
        let mut digis = DigitsIter::new(rest).skip(1).take(3);
        let (one, two, three) = (
            digis.next().unwrap(),
            digis.next().unwrap(),
            digis.next().unwrap(),
        );
        let i = opcode.try_into().unwrap().try_into().unwrap();
        let p_modes = [
            one.try_into().unwrap().try_into().unwrap(),
            two.try_into().unwrap().try_into().unwrap(),
            three.try_into().unwrap().try_into().unwrap(),
        ];
        (i, p_modes)
    }
}

fn mod100<T>(n: &T) -> T
where
    for<'a> &'a T: Rem<T, Output = T>,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    n % 100.try_into().unwrap()
}
