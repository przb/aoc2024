use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Rule {
    before: i32,
    after: i32,
}

struct Update {
    pages: Vec<i32>,
}

struct DailyInput {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> DailyInput {
    let rules = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (lhs, rhs) = line.split_once("|").unwrap();
            Rule {
                before: lhs.parse().unwrap(),
                after: rhs.parse().unwrap(),
            }
        })
        .collect_vec();

    let updates = input
        .lines()
        .skip(rules.len() + 1)
        .map(|line| Update {
            pages: line
                .split(",")
                .map(|token| token.parse().unwrap())
                .collect(),
        })
        .collect();
    DailyInput { rules, updates }
}

// checks if a rule applies, and if so returns if it is valid.
// If a rule does not apply, returns None
fn check_rule(u: &Update, r: &Rule) -> Option<bool> {
    let before = u.pages.iter().find_position(|page| **page == r.before)?;
    let after = u.pages.iter().find_position(|page| **page == r.after)?;

    Some(before < after)
}

#[aoc(day5, part1)]
fn part1(input: &DailyInput) -> i32 {
    input
        .updates
        .iter()
        .filter(|&update| {
            input
                .rules
                .iter()
                .all(|rule| check_rule(update, rule).unwrap_or(true))
        })
        .map(|update| {
            update
                .pages
                .get(update.pages.len() / 2)
                .copied()
                .unwrap_or_default()
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &DailyInput) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    const GIVEN_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    fn get_input() -> String {
        let input_path = "input/2024/day5.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&parse(&get_input())), 4281);
    }

    #[test]
    fn part1_sample_input() {
        assert_eq!(part1(&parse(GIVEN_INPUT)), 143);
    }

    #[test]
    fn part2_example() {
        //assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
