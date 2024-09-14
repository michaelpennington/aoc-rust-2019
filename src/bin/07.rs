use advent_of_code::intcode::Program;
use itertools::Itertools;

advent_of_code::solution!(7);

fn test_permutation(perm: &[i64], comp: &Program<i64>) -> i64 {
    let mut input = 0;
    for &num in perm.iter().take(5) {
        let mut comp = comp.clone();
        comp.input([num, input]);
        input = comp.next().unwrap();
    }
    input
}

fn test_permutation_looping(perm: &[i64], comp: &Program<i64>) -> i64 {
    let mut comps = [
        comp.clone(),
        comp.clone(),
        comp.clone(),
        comp.clone(),
        comp.clone(),
    ];
    comps[0].input([perm[0], 0]);
    comps[1].input([perm[1]]);
    comps[2].input([perm[2]]);
    comps[3].input([perm[3]]);
    comps[4].input([perm[4]]);
    let mut last_seen = [0; 5];

    for i in 0.. {
        let i = i % 5;

        match comps[i].next() {
            Some(v) => {
                comps[(i + 1) % 5].input([v]);
                last_seen[i] = v;
            }
            None => {
                if i == 4 {
                    return last_seen[4];
                }
            }
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<i64> {
    let comp = input.parse::<Program<i64>>().unwrap();
    [0, 1, 2, 3, 4]
        .into_iter()
        .permutations(5)
        .map(|perm| test_permutation(&perm, &comp))
        .max()
}

pub fn part_two(input: &str) -> Option<i64> {
    let comp = input.parse::<Program<i64>>().unwrap();
    [5, 6, 7, 8, 9]
        .into_iter()
        .permutations(5)
        .map(|perm| test_permutation_looping(&perm, &comp))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43210));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(54321));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(65210));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(139629729));
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(18216));
    }
}
