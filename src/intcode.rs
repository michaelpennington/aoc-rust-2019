use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use anyhow::anyhow;
use num_traits::{Num, ToPrimitive};

use crate::util::digits::DigitsIter;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Program<T> {
    code: Vec<T>,
    pc: usize,
    extra_mem: HashMap<usize, T>,
    relative_base: isize,
    input: VecDeque<T>,
    cache: Option<Vec<T>>,
}

impl<T> FromStr for Program<T>
where
    T: Num,
    <T as Num>::FromStrRadixErr: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code = Vec::with_capacity(s.split(',').count());
        for num in s.trim().split(',') {
            code.push(<T as Num>::from_str_radix(num, 10)?);
        }
        Ok(Self {
            code,
            pc: 0,
            extra_mem: HashMap::new(),
            relative_base: 0,
            input: VecDeque::new(),
            cache: None,
        })
    }
}

impl<T> Iterator for Program<T>
where
    T: Num + Clone + Copy + ToPrimitive + PartialOrd + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.process_one() {
                Output::Halted => break None,
                Output::Val(v) => break Some(v),
                Output::None => {}
            }
        }
    }
}

impl<T> Program<T>
where
    T: Num + Clone + Copy + ToPrimitive + PartialOrd + std::fmt::Debug,
{
    pub fn cache(&mut self) {
        self.cache = Some(self.code.clone());
    }

    pub fn reset(&mut self) {
        let cache = self
            .cache
            .as_ref()
            .expect("Must have cached program to reset computer");
        self.code.copy_from_slice(cache);
        self.pc = 0;
        self.extra_mem.clear();
        self.input.clear();
        self.relative_base = 0;
    }

    pub fn input(&mut self, i: impl IntoIterator<Item = T>) {
        self.input.extend(i);
    }

    pub fn get(&self, index: usize) -> T {
        if let Some(val) = self.code.get(index) {
            *val
        } else if let Some(val) = self.extra_mem.get(&index) {
            *val
        } else {
            T::zero()
        }
    }

    pub fn set(&mut self, index: usize, val: T) {
        if let Some(loc) = self.code.get_mut(index) {
            *loc = val;
        } else {
            self.extra_mem.insert(index, val);
        }
    }

    pub fn execute(&mut self) {
        while self.process_one() != Output::Halted {}
    }

    fn process_one(&mut self) -> Output<T> {
        let inst = self.get(self.pc).to_u32().unwrap().try_into().unwrap();
        self.pc += 1;
        self.process_inst(inst)
    }

    fn process_inst(&mut self, i: Instruction) -> Output<T> {
        match i.code {
            Opcode::Add => {
                let x = self.get_with_pmode(self.pc, i.p_modes[0]);
                let y = self.get_with_pmode(self.pc + 1, i.p_modes[1]);
                let addr = self.get_addr_with_pmode(self.pc + 2, i.p_modes[2]);
                self.set(addr, x + y);
                self.pc += 3;
                Output::None
            }
            Opcode::Mul => {
                let x = self.get_with_pmode(self.pc, i.p_modes[0]);
                let y = self.get_with_pmode(self.pc + 1, i.p_modes[1]);
                let addr = self.get_addr_with_pmode(self.pc + 2, i.p_modes[2]);
                self.set(addr, x * y);
                self.pc += 3;
                Output::None
            }
            Opcode::Input => {
                if let Some(inp) = self.input.pop_front() {
                    let addr = self.get_addr_with_pmode(self.pc, i.p_modes[0]);
                    self.set(addr.to_usize().unwrap(), inp);
                    self.pc += 1;
                    Output::None
                } else {
                    panic!("Needed input but none available!")
                }
            }
            Opcode::Output => {
                let val = self.get_with_pmode(self.pc, i.p_modes[0]);
                self.pc += 1;
                Output::Val(val)
            }
            Opcode::Jnz => {
                if !self.get_with_pmode(self.pc, i.p_modes[0]).is_zero() {
                    self.pc = self
                        .get_with_pmode(self.pc + 1, i.p_modes[1])
                        .to_usize()
                        .unwrap();
                } else {
                    self.pc += 2;
                }
                Output::None
            }
            Opcode::Jz => {
                if self.get_with_pmode(self.pc, i.p_modes[0]).is_zero() {
                    self.pc = self
                        .get_with_pmode(self.pc + 1, i.p_modes[1])
                        .to_usize()
                        .unwrap();
                } else {
                    self.pc += 2;
                }
                Output::None
            }
            Opcode::Lt => {
                let addr = self.get_addr_with_pmode(self.pc + 2, i.p_modes[2]);
                if self.get_with_pmode(self.pc, i.p_modes[0])
                    < self.get_with_pmode(self.pc + 1, i.p_modes[1])
                {
                    self.set(addr, T::one());
                } else {
                    self.set(addr, T::zero());
                }
                self.pc += 3;
                Output::None
            }
            Opcode::Eq => {
                let addr = self.get_addr_with_pmode(self.pc + 2, i.p_modes[2]);
                if self.get_with_pmode(self.pc, i.p_modes[0])
                    == self.get_with_pmode(self.pc + 1, i.p_modes[1])
                {
                    self.set(addr, T::one());
                } else {
                    self.set(addr, T::zero());
                }
                self.pc += 3;
                Output::None
            }
            Opcode::Halt => Output::Halted,
            Opcode::RelAdj => {
                let adj = self
                    .get_with_pmode(self.pc, i.p_modes[0])
                    .to_isize()
                    .unwrap();
                self.relative_base += adj;
                self.pc += 1;
                Output::None
            }
        }
    }

    fn get_addr_with_pmode(&self, index: usize, pmode: ParameterMode) -> usize {
        match pmode {
            ParameterMode::Position => self.get(index).to_usize().unwrap(),
            ParameterMode::Immediate => index,
            ParameterMode::Relative => {
                (self.get(index).to_isize().unwrap() + self.relative_base) as usize
            }
        }
    }

    fn get_with_pmode(&self, index: usize, pmode: ParameterMode) -> T {
        self.get(self.get_addr_with_pmode(index, pmode))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<u32> for ParameterMode {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            _ => Err(anyhow!("{value} is not a valid parameter mode")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Instruction {
    p_modes: [ParameterMode; 3],
    code: Opcode,
}

impl TryFrom<u32> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let code = value % 100;
        let code = Opcode::try_from(code)?;
        let mut p_modes = [ParameterMode::Position; 3];
        for (dig, pmode) in DigitsIter::new((value / 100) + 1000)
            .skip(1)
            .zip(&mut p_modes)
        {
            *pmode = dig.try_into().unwrap();
        }
        p_modes.reverse();
        Ok(Self { p_modes, code })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Output<T> {
    Halted,
    Val(T),
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    Halt,
    Jnz,
    Jz,
    Lt,
    Eq,
    RelAdj,
}

impl TryFrom<u32> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::Jnz),
            6 => Ok(Self::Jz),
            7 => Ok(Self::Lt),
            8 => Ok(Self::Eq),
            9 => Ok(Self::RelAdj),
            99 => Ok(Self::Halt),
            _ => Err(anyhow!("{value} is not an acceptable opcode")),
        }
    }
}
