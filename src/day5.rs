use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Copy)]
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

fn assemble_order(rules: &[Rule], update: &Update) -> Update {
    let order = update
        .pages
        .iter()
        .permutations(update.pages.len())
        .find(|permut| {
            rules.iter().all(|rule| {
                check_rule(
                    &Update {
                        pages: permut.iter().copied().copied().collect(),
                    },
                    rule,
                )
                // this fn should only be called on rules that are relevant to the update
                .unwrap()
            })
        })
        .unwrap()
        .iter()
        .copied()
        .copied()
        .collect();

    Update { pages: order }
}

fn update_is_valid(rules: &[Rule], update: &Update) -> bool {
    rules
        .iter()
        // if the value is None, then it is valid in this case
        .all(|rule| check_rule(update, rule).unwrap_or(true))
}

#[aoc(day5, part1)]
fn part1(input: &DailyInput) -> i32 {
    input
        .updates
        .iter()
        .filter(|&update| update_is_valid(&input.rules, update))
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
    // for every update, get only the relevant rules
    // from the relevant rules, reconstruct the ordering
    input
        .updates
        .par_iter()
        .filter(|update| !update_is_valid(&input.rules, update))
        .map(|update| {
            let relevant_rules = input
                .rules
                .iter()
                // if the rule is none, then it is not relevant here, so filter it out (since its inverted)
                .filter(|rule| !check_rule(update, rule).unwrap_or(true))
                .map(|rule| *rule)
                .collect_vec();
            let permut = assemble_order(&relevant_rules, update).pages;
            //println!("permutation: {permut:?}");
            permut[permut.len() / 2]
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
    fn part2_example() {
        //assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(part2(&parse(GIVEN_INPUT)), 123);
    }
}
