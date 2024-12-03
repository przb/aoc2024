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
