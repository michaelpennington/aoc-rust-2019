use advent_of_code::intcode::Program;
use itertools::Itertools;

advent_of_code::solution!(7);

fn test_permutation(perm: &[i32], comp: &Program) -> i32 {
    let mut input = 0;
    for &num in perm.iter().take(5) {
        input = comp.clone().execute_with_input([num, input]);
    }
    input
}

fn test_permutation_looping(perm: &[i32], comp: &Program) -> i32 {
    let mut comps = [
        comp.clone(),
        comp.clone(),
        comp.clone(),
        comp.clone(),
        comp.clone(),
    ];
    let mut perms = [
        vec![perm[0], 0],
        vec![perm[1]],
        vec![perm[2]],
        vec![perm[3]],
        vec![perm[4]],
    ];
    for i in 0.. {
        let i = i % 5;
        match comps[i].execute_with_input_to_vec(perms[i].drain(..)) {
            (None, Some(out)) => {
                if i == 4 {
                    return out;
                } else {
                    perms[i + 1].push(out);
                }
            }
            (Some(v), None) => {
                perms[(i + 1) % 5].extend(v);
            }
            _ => unreachable!(),
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<i32> {
    let comp = input.parse::<Program>().unwrap();
    [0, 1, 2, 3, 4]
        .into_iter()
        .permutations(5)
        .map(|perm| test_permutation(&perm, &comp))
        .max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let comp = input.parse::<Program>().unwrap();
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
