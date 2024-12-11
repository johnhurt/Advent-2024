use std::collections::HashMap;

use advent_of_code::ws;
use itertools::Itertools;
use nom::{character::complete::u64, multi::many1, IResult};

advent_of_code::solution!(11);

fn parse_input(line: &str) -> Vec<u64> {
    let result: IResult<_, _> = many1(ws(u64))(line);
    result.expect("🐸").1
}

pub enum Blink {
    Single(u64),
    Double(u64, u64),
}

fn blink(b: u64) -> Blink {
    if b == 0 {
        Blink::Single(1)
    } else {
        let digits = ((b as f64).log10().floor() + 1.) as u64;
        if digits % 2 == 0 {
            let power_of_ten = 10_u64.pow(digits as u32 / 2);
            let right = b % power_of_ten;
            let left = b / power_of_ten;
            Blink::Double(left, right)
        } else {
            Blink::Single(b * 2024)
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Problem {
    s: u64,
    times: usize,
}

impl Problem {
    fn blink(self) -> (Problem, Option<Problem>) {
        match blink(self.s) {
            Blink::Double(l, r) => (
                Problem {
                    s: l,
                    times: self.times - 1,
                },
                Some(Problem {
                    s: r,
                    times: self.times - 1,
                }),
            ),
            Blink::Single(s) => (
                Problem {
                    s,
                    times: self.times - 1,
                },
                None,
            ),
        }
    }
}

fn blink_fast(stones: Vec<u64>, times: usize) -> u64 {
    let mut cache = HashMap::new();

    let mut to_solve = stones
        .into_iter()
        .map(|s| Problem { s, times })
        .collect_vec();

    let original = to_solve.clone();

    while let Some(problem) = to_solve.pop() {
        match cache.get(&problem).copied() {
            Some(_) => {}
            None if problem.times == 0 => {
                cache.insert(problem, 1);
            }
            None => match problem.blink() {
                (pl, Some(pr)) => {
                    match (cache.get(&pl).copied(), cache.get(&pr).copied()) {
                        (Some(al), Some(ar)) => {
                            cache.insert(problem, al + ar);
                        }
                        (Some(_), None) => {
                            to_solve.extend_from_slice(&[problem, pr]);
                        }
                        (None, Some(_)) => {
                            to_solve.extend_from_slice(&[problem, pl]);
                        }
                        _ => {
                            to_solve.extend_from_slice(&[problem, pl, pr]);
                        }
                    }
                }
                (p, None) => match cache.get(&p).copied() {
                    Some(ans) => {
                        cache.insert(problem, ans);
                    }
                    None => {
                        to_solve.extend_from_slice(&[problem, p]);
                    }
                },
            },
        }
    }

    original
        .iter()
        .map(|s| {
            let Some(ans) = cache.get(s).copied() else {
                unreachable!("Answers are present")
            };
            ans
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);

    let result = blink_fast(stones, 25);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = parse_input(input);

    let result = blink_fast(stones, 75);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
