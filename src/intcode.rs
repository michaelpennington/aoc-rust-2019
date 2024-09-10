use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    program: Vec<usize>,
    pc: usize,
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut program = Vec::with_capacity(s.split(',').count());
        for n in s.trim().split(',') {
            program.push(n.parse().unwrap());
        }
        Ok(Self { program, pc: 0 })
    }
}

impl Program {
    pub fn set(&mut self, index: usize, val: usize) {
        self.program[index] = val;
    }

    pub fn get(&self, index: usize) -> usize {
        self.program[index]
    }

    pub fn execute(&mut self) {
        loop {
            let res = match self.program[self.pc] {
                1 => {
                    let x = self.program[self.program[self.pc + 1]];
                    let y = self.program[self.program[self.pc + 2]];
                    x + y
                }
                2 => {
                    let x = self.program[self.program[self.pc + 1]];
                    let y = self.program[self.program[self.pc + 2]];
                    x * y
                }
                99 => break,
                _ => unreachable!(),
            };
            let addr = self.program[self.pc + 3];
            self.program[addr] = res;
            self.pc += 4;
        }
    }
}
