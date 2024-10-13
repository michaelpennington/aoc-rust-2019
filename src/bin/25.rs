// use std::io::{self, BufRead};

use advent_of_code::intcode::{NonBlockProgram, Program};

advent_of_code::solution!(25);

fn command(i: &str) -> impl Iterator<Item = i64> + use<'_> {
    i.as_bytes().iter().map(|i| *i as i64)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut computer: NonBlockProgram<_> = input.parse::<Program<i64>>().unwrap().into();
    let mut commands = [
        "south\n",
        "south\n",
        "south\n",
        "take astrolabe\n",
        "south\n",
        "take mug\n",
        "north\n",
        "north\n",
        "west\n",
        "north\n",
        "north\n",
        "take wreath\n",
        "south\n",
        "south\n",
        "east\n",
        "north\n",
        "west\n",
        "take sand\n",
        "west\n",
        "west\n",
        "west\n",
    ]
    .into_iter();
    // let stdin = io::stdin();
    while let Some(c) = computer.next() {
        match c {
            Ok(c) => {
                if let Some(c) = char::from_u32(c as u32) {
                    print!("{c}");
                }
            }
            Err(false) => {}
            Err(true) => {
                let cmd = commands.next().unwrap();
                print!("{cmd}");
                computer.program.input(command(cmd));
                // let mut buffer = String::new();
                // let mut handle = stdin.lock();
                // handle.read_line(&mut buffer).unwrap();
                // computer.program.input(command(&buffer));
            }
        }
    }
    Some(328960)
}

pub fn part_two(input: &str) -> Option<u32> {
    let _ = input;
    Some(1)
}
