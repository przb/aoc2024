use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rule::Rule;
use std::str::FromStr;

type Update = Vec<i32>;

struct DailyInput {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

mod rule {
    use std::str::FromStr;

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
    impl ToString for Rule {
        fn to_string(&self) -> String {
            format!("{}|{}", self.before, self.after)
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

fn follows_rule(updates: &Update, rule: &Rule) -> bool {
    try_follows_rule(updates, rule).unwrap()
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

fn assemble_order(rules: &[Rule], _update: &Update) -> Update {
    let mut order = Vec::with_capacity(25);

    rules.iter().for_each(|rule| {
        let before_index = order.iter().position(|val| *val as i32 == rule.before);
        let after_index = order.iter().position(|val| *val as i32 == rule.after);
        if before_index.is_none() && after_index.is_none() {
            // append to end i suppose
            order.push(rule.before);
            order.push(rule.after);
        } else if before_index.is_some() && after_index.is_some() {
            if !follows_rule(&order, &rule) {
                // if order is wrong, then rearrange
                let before_index = before_index.unwrap();
                let after_index = after_index.unwrap();

                let tmp = order.remove(before_index);
                order.insert(after_index, tmp);
            } else {
                // if order is correct, do nothing
            }
        } else if after_index.is_some() {
            // contains after but not before
            let index = after_index.unwrap();
            order.insert(index, rule.before);
        } else if before_index.is_some() {
            // contains before but not after
            let index = before_index.unwrap();
            order.insert(index + 1, rule.after);
        } else {
            unreachable!("this aint supposed to happen")
        }
    });

    order
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
    // for every update, get only the relevant rules
    // from the relevant rules, reconstruct the ordering
    let mut sum = 0;
    let _permuts = input
        .updates
        .iter()
        .filter(|update| update_is_invalid(&input.rules, update))
        .map(|update| {
            let relevant_rules = input
                .rules
                .iter()
                // if the rule is none, then it is not relevant here, so filter it out
                .filter(|rule| try_follows_rule(update, rule).is_some())
                .map(|rule| *rule)
                .collect_vec();
            let permut = assemble_order(&relevant_rules, update);

            assert!(permut.len() & 0x01 == 0x01); // vec must be odd len
            let len = permut.len();
            let mid = permut[len / 2];
            sum += mid;
            //println!("relevant_rules: {relevant_rules:?}");
            //let vec = format!("{permut:?}");
            //println!("mid={mid:3},len={len:2},vec={vec:>100}");
            permut
        })
        .collect_vec();

    sum
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
        assert!(part2(&parse(&get_input())) < 5640);
        assert!(part2(&parse(&get_input())) < 5633);
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(&parse(GIVEN_INPUT)), 123);
    }
}
