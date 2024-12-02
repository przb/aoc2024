use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;

#[allow(dead_code)]
fn custom_parse_h(
    input: &str,
    left: &mut BinaryHeap<Reverse<isize>>,
    rght: &mut BinaryHeap<Reverse<isize>>,
) {
    let mut val = 0;
    input.as_bytes().iter().for_each(|b| match b {
        b' ' => {
            if val != 0 {
                left.push(Reverse(val));
            }
            val = 0;
        }
        b'\n' => {
            rght.push(Reverse(val));
            val = 0;
        }
        b'0'..=b'9' => {
            val *= 10;
            val += *b as isize - 48;
        }
        _ => panic!("im tired of this grampa"),
    });
    if val != 0 {
        rght.push(Reverse(val));
    }
}

#[allow(dead_code)]
fn custom_parse(input: &str, left: &mut Vec<isize>, rght: &mut Vec<isize>) {
    let mut val = 0;
    input.as_bytes().iter().for_each(|b| match b {
        b' ' => {
            if val != 0 {
                left.push(val);
            }
            val = 0;
        }
        b'\n' => {
            rght.push(val);
            val = 0;
        }
        b'0'..=b'9' => {
            val *= 10;
            val += *b as isize - 48;
        }
        _ => panic!("im tired of this grampa"),
    });
    if val != 0 {
        rght.push(val);
    }
}

#[allow(dead_code)]
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

#[aoc(day1, part1, heapified)]
fn part1_h(input: &str) -> isize {
    let mut left = BinaryHeap::with_capacity(1000);
    let mut rght = BinaryHeap::with_capacity(1000);
    custom_parse_h(input, &mut left, &mut rght);

    let mut sum = 0;
    while !(left.is_empty()) {
        let left = left.pop().unwrap().0;
        let right = rght.pop().unwrap().0;
        let diff = isize::abs(left - right);

        sum += diff;
    }

    sum as isize
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> isize {
    let mut left = Vec::with_capacity(1000);
    let mut rght = Vec::with_capacity(1000);
    custom_parse(input, &mut left, &mut rght);
    left.into_iter()
        .sorted()
        .zip(rght.into_iter().sorted())
        .map(|(l, r)| isize::abs(l - r))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut left = Vec::with_capacity(1000);
    let mut rght = Vec::with_capacity(1000);

    custom_parse(input, &mut left, &mut rght);
    let rmap = rght.into_iter().counts();

    left.into_iter()
        .map(|val| val as usize * rmap.get(&val).unwrap_or(&0))
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
