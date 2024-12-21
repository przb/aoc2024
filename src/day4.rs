use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use itertools::Itertools;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

struct DirectionalChars<'a> {
    has_overflowed: bool,
    movement_vec: (isize, isize), // (x, y)
    current_index: usize,
    input: &'a [u8],
}

trait DirectionalIteratorable<'a> {
    fn direction_iter(&'a self, i: usize, direction: Direction) -> DirectionalChars<'a>;
}

impl<'a> DirectionalIteratorable<'a> for &'a str {
    fn direction_iter(&'a self, i: usize, direction: Direction) -> DirectionalChars<'a> {
        DirectionalChars::new(self, direction, i as isize)
    }
}

impl<'a> Iterator for DirectionalChars<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_overflowed {
            return None;
        }
        let char = self.input.get(self.current_index);
        let vec = self.movement_vec.0 + self.movement_vec.1;
        let index = if vec < 0 {
            self.current_index.checked_sub(vec.unsigned_abs())
        } else {
            self.current_index.checked_add(vec as usize)
        };
        match index {
            Some(index) => {
                self.current_index = index;
                char
            }
            None => {
                self.has_overflowed = true;
                char
            }
        }
    }
}

impl<'a> DirectionalChars<'a> {
    fn new(input: &'a str, direction: Direction, i: isize) -> Self {
        let line_len = (input.chars().position(|c| c == '\n').unwrap() + 1) as isize;
        let movement_vec = get_movement_vec(line_len, direction);
        Self {
            has_overflowed: false,
            movement_vec,
            current_index: i as usize,
            input: input.as_bytes(),
        }
    }
}

fn get_movement_vec(line_len: isize, direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (0, -line_len),
        Direction::Down => (0, line_len),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::UpRight => (1, -line_len),
        Direction::UpLeft => (-1, -line_len),
        Direction::DownRight => (1, line_len),
        Direction::DownLeft => (-1, line_len),
    }
}
fn get_movement_vec_sum(line_len: isize, direction: Direction) -> isize {
    let vec = get_movement_vec(line_len, direction);
    vec.0 + vec.1
}
fn is_x_mas(input: &str, index: usize, line_len: isize) -> bool {
    let around = [
        get_movement_vec_sum(line_len, Direction::UpLeft),
        get_movement_vec_sum(line_len, Direction::UpRight),
        get_movement_vec_sum(line_len, Direction::DownRight),
        get_movement_vec_sum(line_len, Direction::DownLeft),
    ];

    let around: Vec<_> = around
        .iter()
        .map(|&sum| {
            let offset = if sum < 0 {
                let sum = sum.unsigned_abs();
                
                index.checked_sub(sum)
            } else {
                index.checked_add(sum as usize)
            }?;
            input.as_bytes().get(offset)
        })
        .collect();
    let ul_to_lr = around[0] == Some(&b'M') && around[2] == Some(&b'S');
    let lr_to_ul = around[2] == Some(&b'M') && around[0] == Some(&b'S');
    let ur_to_ll = around[1] == Some(&b'M') && around[3] == Some(&b'S');
    let ll_to_ur = around[3] == Some(&b'M') && around[1] == Some(&b'S');

    input.as_bytes()[index] == b'A' && (ul_to_lr || lr_to_ul) && (ur_to_ll || ll_to_ur)
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .char_indices()
        .map(|(index, char)| {
            if char == 'X' {
                Direction::iter()
                    .filter(|dir| {
                        let word = input
                            .direction_iter(index, *dir)
                            .map(|val| *val as char)
                            .take(4)
                            .join("");
                        word == "XMAS"
                    })
                    .count()
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let line_len = (input.chars().position(|c| c == '\n').unwrap()) as isize + 1;
    input
        .char_indices()
        .skip(line_len as usize)
        .filter(|(index, _char)| is_x_mas(input, *index, line_len))
        .count()
}

#[cfg(test)]
mod test {

    const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day4.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn get_move_vec_test() {
        let vec = get_movement_vec(11, Direction::UpRight);
        let sum = vec.0 + vec.1;
        assert_eq!(sum, -10)
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT), 18)
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT), 9);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 2662)
    }

    #[test]
    fn part_2_custom_test() {
        let input = "MMM
AAA
SSS";
        assert_eq!(part2(input), 1);

        let input = "SMM
AAA
SSM";
        assert_eq!(part2(input), 1);

        let input = "SMS
AAA
MSM";
        assert_eq!(part2(input), 1);

        let input = "MMS
AAA
MSS";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn part2_real_input() {
        // previous attempt was incorrect
        assert_eq!(part2(&get_input()), 2034);
    }
}
