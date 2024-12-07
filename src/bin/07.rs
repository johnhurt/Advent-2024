use advent_of_code::ws;
use itertools::Itertools;
use nom::character::complete::{char, u64};
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(7);

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let result: IResult<_, _> =
        separated_pair(u64, ws(char(':')), many1(ws(u64)))(line);

    result.expect("🦂").1
}

fn check_2_op_combo(left: u64, right: &[u64], combo: u64) -> bool {
    let ev = right
        .iter()
        .copied()
        .fold((0, 0), |(mask, acc), curr| {
            let mask_zero = (mask == 0) as u64;
            let mask_non_zero = 1 - mask_zero;
            let add = ((mask & combo) > 0) as u64;
            let mul = 1 - add;

            let next_mask = mask_zero + mask_non_zero * (mask << 1);
            let next_acc = mask_zero * curr
                + mask_non_zero
                    * (add * (acc + curr) + mul * (acc.wrapping_mul(curr)));
            (next_mask, next_acc)
        })
        .1;

    ev == left
}

fn valid_p1_math(left: u64, right: &[u64]) -> bool {
    let combinations = 1 << (right.len() - 1);
    (0..combinations).any(|combo| check_2_op_combo(left, right, combo))
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(|(left, right)| valid_p1_math(*left, right.as_slice()))
            .map(|(left, _)| left)
            .sum(),
    )
}

fn concatenate(left: u64, right: u64, right_next_power_of_ten: u64) -> u64 {
    left.wrapping_mul(right_next_power_of_ten) + right
}

fn check_3_op_combo(
    left: u64,
    right_and_npt: &[(u64, u64)],
    combo: u64,
) -> bool {
    let ev = right_and_npt
        .iter()
        .copied()
        .fold((1, combo, 0), |(first, combo, acc), (curr, curr_npt)| {
            let not_first = 1 - first;
            let modulo = combo % 3;
            let add = first + not_first * (modulo == 0) as u64;
            let mul = not_first * (modulo == 1) as u64;
            let concat = not_first * (modulo == 2) as u64;

            let next_combo = combo * first + (combo / 3) * not_first;
            let next_acc = add * (acc + curr)
                + mul * (acc.wrapping_mul(curr))
                + concat * concatenate(acc, curr, curr_npt);
            (0, next_combo, next_acc)
        })
        .2;

    ev == left
}

fn next_power_of_ten(v: u64) -> u64 {
    10_u32.pow(((v as f64).log10().floor() + 1.) as u32) as u64
}

fn valid_p2_math(left: u64, right: &[u64]) -> bool {
    let combinations = 3_u32.pow((right.len() - 1) as u32);
    let right_and_npt = right
        .iter()
        .copied()
        .map(|v| (v, next_power_of_ten(v)))
        .collect_vec();
    (0..combinations)
        .any(|combo| check_3_op_combo(left, &right_and_npt, combo as u64))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(parse_line)
            .filter(|(left, right)| {
                valid_p1_math(*left, right.as_slice())
                    || valid_p2_math(*left, right)
            })
            .map(|(left, _)| left)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
