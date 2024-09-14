use advent_of_code::intcode::Program;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i32> {
    let mut computer = input.parse::<Program<i32>>().unwrap();
    computer.input([1]);
    let mut peekable = computer.peekable();
    while let Some(val) = peekable.next() {
        if peekable.peek().is_none() {
            return Some(val);
        }
        assert_eq!(val, 0);
    }
    None
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut computer = input.parse::<Program<i32>>().unwrap();
    computer.input([5]);
    Some(computer.next().unwrap())
}
