use advent_of_code::intcode::Program;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut comp = input.parse::<Program<i64>>().unwrap();
    comp.input([1]);
    let out = comp.next().unwrap();
    Some(out)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut comp = input.parse::<Program<i64>>().unwrap();
    comp.input([2]);
    let out = comp.next().unwrap();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(109));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1219070632396864));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1125899906842624));
    }
}
