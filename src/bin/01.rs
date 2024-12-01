use std::collections::HashMap;

use advent_of_code::ws;
use nom::{character::complete::u32, sequence::pair, IResult};

advent_of_code::solution!(1);

fn parse_line(line: &str) -> (u32, u32) {
    let result: IResult<_, _> = pair(ws(u32), ws(u32))(line);
    result.expect("😎").1
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) =
        input.lines().map(parse_line).unzip();

    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<_>, Vec<_>) = input.lines().map(parse_line).unzip();

    let mut right_counts = HashMap::<u32, u32>::new();

    for v in right.into_iter() {
        right_counts
            .entry(v)
            .and_modify(|existing| *existing += 1)
            .or_insert(1);
    }

    let result = left
        .into_iter()
        .map(|left| {
            (right_counts.get(&left).copied().unwrap_or_default(), left)
        })
        .map(|(left, right)| left * right)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
