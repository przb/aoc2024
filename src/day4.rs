use std::isize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use itertools::Itertools;

#[derive(EnumIter, Clone, Copy, Debug)]
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
            self.current_index.checked_sub(vec.abs() as usize)
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
        let movement_vec = match direction {
            Direction::Up => (0, line_len * -1),
            Direction::Down => (0, line_len),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpRight => (1, line_len * -1),
            Direction::UpLeft => (-1, line_len * -1),
            Direction::DownRight => (1, line_len),
            Direction::DownLeft => (-1, line_len),
        };
        Self {
            has_overflowed: false,
            movement_vec,
            current_index: i as usize,
            input: input.as_bytes(),
        }
    }
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
    fn part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT), 18)
    }

    #[test]
    fn part2_sample_input() {
        //assert_eq!(part2(SAMPLE_INPUT), 4)
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 2662)
    }

    #[test]
    fn part2_real_input() {
        //assert_eq!(part2(&get_input()), 514)
    }
}
