use aoc_runner_derive::aoc;
use itertools::Itertools;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[inline(always)]
fn get_vec(direction: &Direction, line_len: isize) -> (isize, isize) {
    match direction {
        Direction::Up => (0, -1 * line_len),
        Direction::Right => (1, 0),
        Direction::Down => (0, line_len),
        Direction::Left => (-1, 0),
    }
}

#[inline(always)]
fn rotate(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

type TraverserItem = (u8, usize);
struct Traverser<'a> {
    next_item: Option<TraverserItem>,
    line_len: usize,
    direction: Direction,
    move_amt: isize,
    current_idx: usize,
    input: &'a str,
}

trait Traversable<'a> {
    fn traverse(&self) -> Traverser<'a>;
}

impl<'a> Traverser<'a> {
    fn new(input: &'a str) -> Self {
        let line_len = input.lines().next().unwrap().len() + 1;
        let current_location = input.as_bytes().iter().position(|c| *c == b'^').unwrap();
        let move_vec = get_vec(&Direction::Up, line_len.try_into().unwrap());
        Self {
            direction: Direction::Up,
            move_amt: move_vec.0 + move_vec.1,
            current_idx: current_location,
            line_len,
            input,
            next_item: input
                .as_bytes()
                .get(current_location)
                .map(|byte| (*byte, current_location)),
        }
    }

    fn progress(&mut self) -> Option<(u8, usize)> {
        let mut new_idx = self.current_idx.checked_add_signed(self.move_amt)?;
        let mut next_item = self.input.as_bytes().get(new_idx)?;
        //println!("new_idx: {new_idx}, next_item: {:?}", *next_item as char);
        match next_item {
            // TODO i could change the interface of this iterator to return \n and just use the
            // .take_while fn
            b'\n' => None,
            b'#' => {
                while *next_item == b'#' {
                    // need to rotate, and update the state
                    self.direction = rotate(&self.direction);
                    let move_vec = get_vec(&self.direction, self.line_len.try_into().unwrap());
                    self.move_amt = move_vec.0 + move_vec.1;

                    new_idx = self.current_idx.checked_add_signed(self.move_amt)?;
                    next_item = self.input.as_bytes().get(new_idx)?;
                    self.current_idx = new_idx;
                    //println!(
                    //    "INNER: new_idx: {}, next_item: {:?}",
                    //    new_idx, *next_item as char
                    //);
                }
                Some((*next_item, new_idx))
            }
            _ => {
                // this includes the . and the ^ characters
                self.current_idx = new_idx;
                Some((*next_item, new_idx))
            }
        }
    }
}

impl<'a> Traversable<'a> for &'a str {
    fn traverse(&self) -> Traverser<'a> {
        Traverser::new(&self)
    }
}

impl Iterator for Traverser<'_> {
    // The byte of the char, and the index of the char
    type Item = (u8, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.next_item;
        self.next_item = self.progress();
        item
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    input
        .traverse()
        .map(|(_ch, idx)| idx)
        .sorted()
        .dedup()
        .count()
        .try_into()
        .unwrap()
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

    fn get_input() -> String {
        let input_path = "input/2024/day6.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_given_input() {
        assert_eq!(part1(&SAMPLE_INPUT), 41);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 5331);
    }

    #[test]
    fn part2_example() {
        //assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
