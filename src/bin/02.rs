use advent_of_code::intcode::Program;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut program: Program<u32> = input.parse().unwrap();
    program.set(1, 12);
    program.set(2, 2);
    program.execute();
    Some(program.get(0))
}

pub fn part_two(input: &str) -> Option<u32> {
    let program: Program<u32> = input.parse().unwrap();
    for (n, v) in (0..100).flat_map(|x| (0..100).map(move |y| (x, y))) {
        let mut program = program.clone();
        program.set(1, n);
        program.set(2, v);
        program.execute();
        if program.get(0) == 19690720 {
            return Some(100 * n + v);
        }
    }
    None
}
