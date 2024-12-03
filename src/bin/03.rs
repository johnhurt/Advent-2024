use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{
    character::complete::char,
    sequence::{terminated, tuple},
    IResult,
};

advent_of_code::solution!(3);

fn parse_mul(to_check: &str) -> IResult<&'_ str, (i64, i64)> {
    preceded(
        tag("mul("),
        tuple((terminated(i64, char(',')), terminated(i64, char(')')))),
    )(to_check)
}

fn try_parse_mul(to_check: &str) -> Option<(i64, i64)> {
    parse_mul(to_check).map(|(_, r)| r).ok()
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut to_check = vec![];

    let mut next = input;
    while let Some(i) = next.find("mul(") {
        to_check.push(&next[i..]);
        next = &next[i + 4..];
    }

    let result = to_check
        .into_iter()
        .filter_map(try_parse_mul)
        .map(|(l, r)| l * r)
        .sum();

    Some(result)
}

#[derive(Debug)]
enum Inst {
    Do,
    Dont,
    Mul(i64, i64),
}

fn try_parse_inst(input: &str) -> IResult<&'_ str, Inst> {
    alt((
        map(tag("don't()"), |_| Inst::Dont),
        map(tag("do()"), |_| Inst::Do),
        map(parse_mul, |(l, r)| Inst::Mul(l, r)),
    ))(input)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut to_check = vec![];

    let mut next = input;
    while let Some(i) = next.find(['m', 'd']) {
        to_check.push(&next[i..]);
        next = &next[i + 1..];
    }

    let insts = to_check
        .into_iter()
        .filter_map(|s| try_parse_inst(s).ok())
        .map(|(_, r)| r)
        .collect_vec();

    let (_, result) =
        insts
            .into_iter()
            .fold((true, 0), |(next_do, sum), inst| match inst {
                Inst::Do => (true, sum),
                Inst::Mul(l, r) if next_do => (true, sum + l * r),
                _ => (false, sum),
            });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
