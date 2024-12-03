#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|lines| {
            let old_line = lines
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap());
            let mut line = lines
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap());

            let sorted = line.clone().is_sorted() || line.clone().rev().is_sorted();
            let _ = line.next();
            sorted
                && old_line
                    .zip(line)
                    .map(|(prev, curr)| prev.abs_diff(curr))
                    .all(|val| val <= 3 && val >= 1)
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    0
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
        assert_eq!(part2(&get_input()), 0)
    }
}
