use advent_of_code::ws;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u64},
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Puzzle {
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    x: i128,
    y: i128,
}

impl Puzzle {
    fn from_parsed(
        button_a: (u64, u64),
        button_b: (u64, u64),
        prize: (u64, u64),
    ) -> Self {
        Puzzle {
            ax: button_a.0 as i128,
            ay: button_a.1 as i128,
            bx: button_b.0 as i128,
            by: button_b.1 as i128,
            x: prize.0 as i128,
            y: prize.1 as i128,
        }
    }

    // ALGEBRA 💪
    fn solve(self) -> Option<(i128, i128)> {
        let Puzzle {
            ax,
            ay,
            bx,
            by,
            x,
            y,
        } = self;

        // let b = (x - (y / ay) * ax) / (bx - (by / ay) * ax);
        let b_num = ay * x - y * ax;
        let b_denom = ay * bx - by * ax;

        let b = (b_num % b_denom == 0).then_some(b_num / b_denom)?;

        // let a = y / ay - (b / ay);
        let a_num = y - b * by;
        let a_denom = ay;

        (a_num % a_denom == 0).then_some((a_num / a_denom, b))
    }
}

fn parse_button(input: &str) -> IResult<&'_ str, (u64, u64)> {
    preceded(
        tuple((tag("Button "), alt((char('A'), char('B'))), tag(": X+"))),
        separated_pair(u64, tag(", Y+"), u64),
    )(input)
}

fn parse_prize(input: &str) -> IResult<&'_ str, (u64, u64)> {
    preceded(tag("Prize: X="), separated_pair(u64, tag(", Y="), u64))(input)
}

fn parse_puzzle(input: &str) -> Option<(&'_ str, Puzzle)> {
    let result: Result<_, _> = map(
        tuple((ws(parse_button), ws(parse_button), ws(parse_prize))),
        |(a, b, p)| Puzzle::from_parsed(a, b, p),
    )(input);

    result.ok()
}

pub fn part_one(mut input: &str) -> Option<i128> {
    let mut total = 0;
    while let Some((next_input, puzzle)) = parse_puzzle(input) {
        input = next_input;
        if let Some((a, b)) = puzzle.solve() {
            total += 3 * a + b;
        }
    }

    Some(total)
}

pub fn part_two(mut input: &str) -> Option<i128> {
    let mut total = 0;
    while let Some((next_input, mut puzzle)) = parse_puzzle(input) {
        puzzle.x += 10000000000000;
        puzzle.y += 10000000000000;
        input = next_input;
        if let Some((a, b)) = puzzle.solve() {
            total += 3 * a + b;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
