use itertools::Itertools;

#[allow(dead_code)]
fn builtin_parse(input: &str, left: &mut Vec<isize>, rght: &mut Vec<isize>) {
    let _ = input.lines().for_each(|line| {
        let mut line_iter = line.split_ascii_whitespace();
        left.push(line_iter.next().unwrap().parse().unwrap());
        rght.push(line_iter.next().unwrap().parse().unwrap());
    });
}

#[allow(dead_code)]
#[inline(always)]
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

    // file not ended by a \n
    if val != 0 {
        rght.push(val);
    }
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
