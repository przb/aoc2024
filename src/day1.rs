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
