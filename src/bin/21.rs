use advent_of_code::intcode::Program;
use strum::IntoStaticStr;

advent_of_code::solution!(21);

#[derive(Clone, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
enum Op {
    And,
    Or,
    Not,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WriteRegister {
    J,
    T,
}

impl WriteRegister {
    fn as_i32(&self) -> i32 {
        match self {
            WriteRegister::J => 'J' as i32,
            WriteRegister::T => 'T' as i32,
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ReadRegister {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    T,
}

impl ReadRegister {
    fn as_i32(&self) -> i32 {
        match self {
            ReadRegister::A => 'A' as i32,
            ReadRegister::B => 'B' as i32,
            ReadRegister::C => 'C' as i32,
            ReadRegister::D => 'D' as i32,
            ReadRegister::E => 'E' as i32,
            ReadRegister::F => 'F' as i32,
            ReadRegister::G => 'G' as i32,
            ReadRegister::H => 'H' as i32,
            ReadRegister::I => 'I' as i32,
            ReadRegister::J => 'J' as i32,
            ReadRegister::T => 'T' as i32,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Command {
    op: Op,
    arg1: ReadRegister,
    arg2: WriteRegister,
}

impl IntoIterator for Command {
    type Item = i32;

    type IntoIter = CommandIter;

    fn into_iter(self) -> Self::IntoIter {
        let word = self.op.into();
        CommandIter {
            command: self,
            word,
            pos: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct CommandIter {
    command: Command,
    word: &'static str,
    pos: usize,
}

impl Iterator for CommandIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 2 || (self.pos == 2 && self.command.op != Op::Or) {
            let out: char = self.word.as_bytes()[self.pos].into();
            self.pos += 1;
            Some(out as i32)
        } else {
            let pos = if self.command.op == Op::Or {
                self.pos + 1
            } else {
                self.pos
            };
            self.pos += 1;
            match pos {
                3 | 5 => Some(' ' as i32),
                4 => Some(self.command.arg1.as_i32()),
                6 => Some(self.command.arg2.as_i32()),
                7 => Some('\n' as i32),
                _ => None,
            }
        }
    }
}

macro_rules! command {
    (AND $arg1:ident $arg2:ident) => {
        Command {
            op: Op::And,
            arg1: ReadRegister::$arg1,
            arg2: WriteRegister::$arg2,
        }
    };
    (OR $arg1:ident $arg2:ident) => {
        Command {
            op: Op::Or,
            arg1: ReadRegister::$arg1,
            arg2: WriteRegister::$arg2,
        }
    };
    (NOT $arg1:ident $arg2:ident) => {
        Command {
            op: Op::Not,
            arg1: ReadRegister::$arg1,
            arg2: WriteRegister::$arg2,
        }
    };
}

macro_rules! program {
    ($($op:ident $arg1:ident $arg2:ident);* $(;)?) => {{
        #[allow(unused_mut)]
        let mut out: Vec<Command> = vec![
            $(command!($op $arg1 $arg2)),*
        ];
        out.into_iter().flatten().chain("WALK\n".as_bytes().iter().map(|&b| b as i32))
    }};
}

macro_rules! program_pt2 {
    ($($op:ident $arg1:ident $arg2:ident);* $(;)?) => {{
        #[allow(unused_mut)]
        let mut out: Vec<Command> = vec![
            $(command!($op $arg1 $arg2)),*
        ];
        out.into_iter().flatten().chain("RUN\n".as_bytes().iter().map(|&b| b as i32))
    }};
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut computer = input.parse::<Program<i32>>().unwrap();
    let program = program!(
        NOT A T;
        OR T J;
        NOT B T;
        OR T J;
        NOT C T;
        OR T J;
        AND D J;
    );
    computer.input(program);
    for b in computer {
        if let Some(c) = char::from_u32(b as u32) {
            print!("{c}");
        } else {
            return Some(b);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut computer = input.parse::<Program<i32>>().unwrap();
    let program = program_pt2!(
        NOT A T;
        OR T J;
        NOT B T;
        OR T J;
        NOT C T;
        OR T J;
        NOT D T;
        OR E T;
        OR H T;
        AND D J;
        AND T J;
    );
    computer.input(program);
    for b in computer {
        if let Some(c) = char::from_u32(b as u32) {
            print!("{c}");
        } else {
            return Some(b);
        }
    }
    None
}
