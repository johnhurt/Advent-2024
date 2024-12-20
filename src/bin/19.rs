use std::collections::HashMap;

use advent_of_code::ws;
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, char},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use regex::Regex;

advent_of_code::solution!(19);

fn parse_pattern(input: &str) -> IResult<&'_ str, Vec<&'_ str>> {
    separated_list1(char(','), ws(alpha1))(input)
}

fn parse_input(input: &str) -> (Vec<&'_ str>, Vec<&'_ str>) {
    let result: IResult<_, _> =
        tuple((ws(parse_pattern), many1(ws(alpha1))))(input);

    result.expect("😤").1
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);

    let rx = Regex::new(&format!("^(?:{})+$", towels.into_iter().join("|")))
        .expect("🤣");
    let result = patterns.into_iter().filter(|p| rx.is_match(p)).count();

    Some(result)
}

fn count_towel_combos(
    pattern: &str,
    towels_by_ending: &HashMap<u8, Vec<&str>>,
    prev: &[usize],
) -> usize {
    let last = pattern.as_bytes()[pattern.len() - 1];

    towels_by_ending
        .get(&last)
        .into_iter()
        .flatten()
        .filter(|towel| pattern.ends_with(**towel))
        .map(|towel| prev[pattern.len() - towel.len()])
        .sum()
}

fn count_all_towel_combos(
    pattern: &str,
    towels_by_ending: &HashMap<u8, Vec<&str>>,
) -> usize {
    let mut prev = Vec::with_capacity(pattern.len());
    prev.push(1);

    (1..=pattern.len())
        .map(|e| &pattern[..e])
        .for_each(|sub_pattern| {
            prev.push(count_towel_combos(sub_pattern, towels_by_ending, &prev));
        });

    *prev.last().unwrap()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    let towels_by_ending = towels
        .iter()
        .map(|towel| (towel.as_bytes()[towel.len() - 1], *towel))
        .into_group_map();

    let result = patterns
        .into_iter()
        .map(|pattern| count_all_towel_combos(pattern, &towels_by_ending))
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
