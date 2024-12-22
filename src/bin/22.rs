use advent_of_code::ws;
use itertools::Itertools;
use nom::{character::complete::u64, multi::many1, IResult};

advent_of_code::solution!(22);

fn parse_input(input: &str) -> Vec<u64> {
    let result: IResult<_, _> = many1(ws(u64))(input);
    result.expect("🥊").1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Secret(u64);

impl Iterator for Secret {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.mix(self.0 * 64);
        self.prune();
        self.mix(self.0 / 32);
        self.prune();
        self.mix(self.0 * 2048);
        self.prune();

        Some(*self)
    }
}

impl Secret {
    fn mix(&mut self, v: u64) {
        self.0 ^= v;
    }

    fn prune(&mut self) {
        self.0 %= 16777216;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse_input(input)
        .into_iter()
        .map(Secret)
        .map(|mut s| s.nth(1999).unwrap().0)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    parse_input(input)
        .into_iter()
        .map(Secret)
        .flat_map(|s| {
            s.take(2000)
                .map(|s| (s.0 % 10) as i8)
                .tuple_windows::<(_, _)>()
                .map(|(l, r)| (r as usize, r - l))
                .tuple_windows::<(_, _, _, _)>()
                .map(|((_, d1), (_, d2), (_, d3), (p, d4))| {
                    ((d1, d2, d3, d4), p)
                })
                .into_group_map()
                .into_iter()
                .map(|(seq, vs)| (seq, vs.first().copied().unwrap_or_default()))
        })
        .into_group_map()
        .into_values()
        .map(|vs| vs.into_iter().sum())
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let mut v = Secret(123);
        assert_eq!(v.next().unwrap(), Secret(15887950));
        assert_eq!(v.next().unwrap(), Secret(16495136));
        assert_eq!(v.next().unwrap(), Secret(527345));
        assert_eq!(v.next().unwrap(), Secret(704524));
    }

    #[test]
    fn test_mix() {
        let mut v = Secret(42);
        v.mix(15);
        assert_eq!(v, Secret(37));
    }

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
