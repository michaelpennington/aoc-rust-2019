use advent_of_code::util::digits::DigitsIter;

advent_of_code::solution!(4);

fn is_valid(n: &u32) -> bool {
    let mut two_adjacent = false;
    let mut prev_digit = None;
    for digit in DigitsIter::new(*n) {
        two_adjacent |= prev_digit.is_some_and(|d| d == digit);
        if prev_digit.is_some_and(|d| d > digit) {
            return false;
        }
        prev_digit = Some(digit);
    }
    two_adjacent
}

fn is_valid2(n: &u32) -> bool {
    let mut digits = DigitsIter::new(*n);
    let (d0, d1, d2, d3, d4, d5) = (
        digits.next().unwrap(),
        digits.next().unwrap(),
        digits.next().unwrap(),
        digits.next().unwrap(),
        digits.next().unwrap(),
        digits.next().unwrap(),
    );
    (d0 <= d1 && d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5)
        && ((d0 == d1 && d1 != d2)
            || (d0 != d1 && d1 == d2 && d2 != d3)
            || (d1 != d2 && d2 == d3 && d3 != d4)
            || (d2 != d3 && d3 == d4 && d4 != d5)
            || (d3 != d4 && d4 == d5))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (from, to) = input
        .trim()
        .split_once('-')
        .map(|(from, to)| (from.parse::<u32>().unwrap(), to.parse::<u32>().unwrap()))
        .unwrap();
    Some((from..=to).filter(is_valid).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (from, to) = input
        .trim()
        .split_once('-')
        .map(|(from, to)| (from.parse::<u32>().unwrap(), to.parse::<u32>().unwrap()))
        .unwrap();
    Some((from..=to).filter(is_valid2).count() as u32)
}
