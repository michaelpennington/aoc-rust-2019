use advent_of_code::intcode::Program;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i32> {
    let mut computer = input.parse::<Program>().unwrap();
    Some(computer.execute_with_input(std::iter::once(1)))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut computer = input.parse::<Program>().unwrap();
    Some(computer.execute_with_input(std::iter::once(5)))
}
