advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<u32>().unwrap() / 3 - 2)
            .sum(),
    )
}

fn calc_fuel(m: u32) -> u32 {
    let logm = (m + 3).ilog(3);
    (1..=logm)
        .map(|n| ((m + 3 - 3u32.pow(n)) / 3u32.pow(n)).saturating_sub(2))
        .sum::<u32>()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| calc_fuel(l.parse::<u32>().unwrap()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34241));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51316));
    }
}
