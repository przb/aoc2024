use itertools::Itertools;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> isize {
    let mut left: Vec<isize> = vec![];
    let mut rght: Vec<isize> = vec![];
    input.lines().for_each(|line| {
        let mut line = line.split_whitespace();
        left.push(line.next().unwrap().parse().unwrap());
        rght.push(line.next().unwrap().parse().unwrap());
    });
    left.sort();
    rght.sort();
    left.iter().zip(rght).map(|(l, r)| isize::abs(l - r)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut left: Vec<isize> = vec![];
    let mut rght: Vec<isize> = vec![];
    input.lines().for_each(|line| {
        let mut line = line.split_whitespace();
        left.push(line.next().unwrap().parse().unwrap());
        rght.push(line.next().unwrap().parse().unwrap());
    });

    let rmap = rght.into_iter().counts();

    left.iter().map(|val| *val as usize * rmap.get(val).unwrap_or(&0)).sum()
}
