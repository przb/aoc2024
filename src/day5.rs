use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rule::Rule;
use std::{cmp::Ordering, str::FromStr};

type Update = Vec<i32>;

struct DailyInput {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

mod rule {
    use std::{fmt::Display, str::FromStr};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub(crate) struct Rule {
        pub(crate) before: i32,
        pub(crate) after: i32,
    }

    #[derive(Debug)]
    pub(crate) enum RuleParseError {
        ParseIntError,
        ParsePipeError,
    }
    impl FromStr for Rule {
        type Err = RuleParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (lhs, rhs) = s.split_once("|").ok_or(RuleParseError::ParsePipeError)?;
            Ok(Rule {
                before: lhs.parse().map_err(|_| RuleParseError::ParseIntError)?,
                after: rhs.parse().map_err(|_| RuleParseError::ParseIntError)?,
            })
        }
    }
    impl Display for Rule {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}|{}", self.before, self.after)
        }
    }
}
// checks if a rule applies, and if so returns if it is valid.
// If a rule does not apply, returns None
fn try_follows_rule(updates: &Update, r: &Rule) -> Option<bool> {
    let before = updates.iter().find_position(|page| **page == r.before)?;
    let after = updates.iter().find_position(|page| **page == r.after)?;

    Some(before < after)
}

// Given a list of rules and an update, check to see if the update follows the rules
fn update_is_valid(rules: &[Rule], update: &Update) -> bool {
    rules
        .iter()
        // if the value is None, then it is valid in this case
        .all(|rule| try_follows_rule(update, rule).unwrap_or(true))
}

// Given a list of rules and an update, check to see if the update breaks the rules
fn update_is_invalid(rules: &[Rule], update: &Update) -> bool {
    rules
        .iter()
        .any(|rule| match try_follows_rule(update, rule) {
            Some(follows_rule) => !follows_rule,
            None => false,
        })
}

#[aoc_generator(day5)]
fn parse(input: &str) -> DailyInput {
    let rules = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| Rule::from_str(line).unwrap())
        .collect_vec();

    let updates = input
        .lines()
        .skip(rules.len() + 1)
        .map(|line| {
            line.split(",")
                .map(|token| token.parse().unwrap())
                .collect()
        })
        .collect();
    DailyInput { rules, updates }
}

#[aoc(day5, part1)]
fn part1(input: &DailyInput) -> i32 {
    input
        .updates
        .iter()
        .filter(|&update| update_is_valid(&input.rules, update))
        .map(|update| update.get(update.len() / 2).copied().unwrap_or_default())
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &DailyInput) -> i32 {
    let mut rule_lookup = [[0; 100]; 100];
    input
        .rules
        .iter()
        .for_each(|rule| rule_lookup[rule.before as usize][rule.after as usize] = 1);

    input
        .updates
        .iter()
        .filter(|update| update_is_invalid(&input.rules, update))
        .map(|update| {
            update
                .iter()
                .sorted_by(|a, b| {
                    if rule_lookup[**a as usize][**b as usize] == 1 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .nth(update.len() / 2)
                .unwrap()
        })
        .sum()
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
    fn part2_real_input() {
        assert_eq!(part2(&parse(&get_input())), 5466);
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(&parse(GIVEN_INPUT)), 123);
    }
}
