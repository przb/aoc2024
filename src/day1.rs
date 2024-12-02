use itertools::Itertools;

fn get_both<'a>(
    input: &'a str,
) -> (
    impl Iterator<Item = isize> + 'a,
    impl Iterator<Item = isize> + 'a,
) {
    let mut left = Vec::with_capacity(1000);
    let mut rght = Vec::with_capacity(1000);
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|c| c.parse().unwrap()))
        .for_each(|mut line| {
            left.push(line.next().unwrap());
            rght.push(line.next().unwrap());
        });
    (left.into_iter(), rght.into_iter())
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> isize {
    let (left, rght) = get_both(input);
    left.into_iter()
        .sorted()
        .zip(rght.into_iter().sorted())
        .map(|(l, r)| isize::abs(l - r))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let (left, rght) = get_both(input);
    let rmap = rght.counts();

    left.map(|val| val as usize * rmap.get(&val).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day1.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1530215)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 26800609)
    }
}
