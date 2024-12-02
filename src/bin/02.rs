use itertools::Itertools;
use nom::{
    character::complete::{char, i32},
    multi::{many1, separated_list1},
    IResult,
};

advent_of_code::solution!(2);

fn parse_line(line: &str) -> Vec<i32> {
    let result: IResult<_, _> = separated_list1(many1(char(' ')), i32)(line);
    result.expect("😎").1
}

fn safe(values: &[i32]) -> Result<(), usize> {
    let mut iter = values.iter().copied().enumerate();
    let mut prev = iter.next().unwrap().1;
    let mut prev_diff = None;

    for (i, v) in iter {
        let diff = v - prev;

        match (prev_diff, diff) {
            (Some(1..=3), 1..=3) => {}
            (Some(-3..=-1), -3..=-1) => {}
            (None, -3 | -2 | -1 | 1 | 2 | 3) => {}
            (_, _) => return Err(i - 1),
        }

        prev = v;
        prev_diff = Some(diff);
    }

    Ok(())
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .map(parse_line)
        .filter(|vals| safe(vals).is_ok())
        .count();

    Some(result)
}

/// This is called if the original list is not found to be safe. We Specifically
/// check the safety around the detected failure point. If one of the values
/// around the failure point can be removed to make the list safe, then return
/// true
fn salvageable(values: &[i32], failure_point: usize) -> bool {
    let possible_removals = [
        failure_point.saturating_sub(1),
        failure_point,
        failure_point + 1,
    ];

    let mut fixed = Vec::with_capacity(values.len() - 1);

    possible_removals.into_iter().unique().any(|to_remove| {
        fixed.clear();

        fixed.extend(
            values
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(i, v)| (i != to_remove).then_some(v)),
        );

        safe(&fixed).is_ok()
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .map(parse_line)
        .map(|vals| (safe(&vals), vals))
        .filter(|(prev_safety, vals)| match prev_safety {
            Ok(()) => true,
            Err(i) => salvageable(vals, *i),
        })
        .count();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
