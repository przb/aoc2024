use std::ops::Sub;

use aoc_runner_derive::aoc;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_vec(direction: &Direction, line_len: isize) -> (isize, isize) {
    match direction {
        Direction::Up => (0, -1 * line_len),
        Direction::Right => (1, 0),
        Direction::Down => (0, line_len),
        Direction::Left => (-1, 0),
    }
}

fn rotate(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let line_len = input.lines().nth(0).unwrap().len();

    let mut curr_loc = input.as_bytes().iter().position(|c| *c == b'^');
    let mut count = 0;
    let mut dir = Direction::Up;

    while let Some(mut loc) = curr_loc {
        let loc_vec = get_vec(&dir, line_len as isize);
        let loc_vec = loc_vec.0 + loc_vec.1;
        loc = if loc_vec < 0 {
            loc - loc_vec.unsigned_abs()
        } else {
            loc + loc_vec.unsigned_abs()
        };
        curr_loc = loc.checked_add_signed(loc_vec);
        let char_at = input.as_bytes()[loc];
        if char_at != b'.' {
            dir = rotate(dir)
        } else {
            count += 1;
        }
    }

    count
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const SAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[allow(dead_code)]
    fn get_input() -> String {
        let input_path = "input/2024/day6.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_given_input() {
        assert_eq!(part1(&SAMPLE_INPUT), 41);
    }

    #[test]
    fn part2_example() {
        //assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
