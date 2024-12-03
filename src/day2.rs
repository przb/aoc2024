use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

// returns true if the line is valid, false if not
// valid if all of the following:
//  - either ascending or decending
//  - the differences between each sequential number is > 0 and < 4
fn line_is_valid(line: &str, safeties: usize) -> bool {
    let num_nums = line.split_ascii_whitespace().count();
    line.split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .combinations(num_nums - safeties)
        .any(|line| {
            let old_line = line.iter();

            let sorted = line.is_sorted() || line.iter().rev().is_sorted();
            sorted
                && old_line
                    .zip(line.iter().skip(1))
                    .map(|(prev, curr)| prev.abs_diff(*curr))
                    .all(|val| val <= 3 && val >= 1)
        })
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input.par_lines().filter(|line| line_is_valid(line, 0)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input.par_lines().filter(|line| line_is_valid(line, 1)).count()
}

#[cfg(test)]
mod test {

    const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day2.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT), 2)
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT), 4)
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 463)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 514)
    }
}
