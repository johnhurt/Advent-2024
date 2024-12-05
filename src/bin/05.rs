use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, u64},
    multi::separated_list1,
};
use std::{cmp::Ordering, collections::HashSet};

use nom::{sequence::separated_pair, IResult};

advent_of_code::solution!(5);

fn parse_rule_line(line: &str) -> IResult<&'_ str, (u64, u64)> {
    separated_pair(u64, char('|'), u64)(line)
}

fn parse_update_line(line: &str) -> IResult<&'_ str, Vec<u64>> {
    separated_list1(char(','), u64)(line)
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let result: IResult<_, _> = separated_pair(
        separated_list1(char('\n'), parse_rule_line),
        tag("\n\n"),
        separated_list1(char('\n'), parse_update_line),
    )(input);

    result.expect("😓").1
}

fn check_order(update: &[u64], orders: &HashSet<(u64, u64)>) -> bool {
    update
        .iter()
        .copied()
        .tuple_windows::<(_, _)>()
        .all(|key| orders.contains(&key))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    let orders = rules.into_iter().collect::<HashSet<_>>();

    let result = updates
        .into_iter()
        .filter(|update| check_order(update, &orders))
        .map(|update| update[update.len() / 2])
        .sum();

    Some(result)
}

fn sort(update: &mut [u64], orders: &HashSet<(u64, u64)>) {
    update.sort_by(|l, r| {
        if orders.contains(&(*l, *r)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    let orders = rules.into_iter().collect::<HashSet<_>>();

    let result = updates
        .into_iter()
        .filter(|update| !check_order(update, &orders))
        .map(|mut update| {
            sort(&mut update, &orders);
            update
        })
        .map(|update| update[update.len() / 2])
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
