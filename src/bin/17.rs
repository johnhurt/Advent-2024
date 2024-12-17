use advent_of_code::ws;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, i64, u8},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use strum::FromRepr;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, FromRepr, PartialEq, Eq)]
#[repr(u8)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct State<'a> {
    abc: [i64; 3],
    p: &'a [u8],
    i: u8,
}

fn parse_register(input: &str) -> IResult<&'_ str, i64> {
    preceded(tuple((tag("Register "), anychar, tag(": "))), i64)(input)
}

fn parse_registers(input: &str) -> IResult<&'_ str, [i64; 3]> {
    map(
        tuple((ws(parse_register), ws(parse_register), ws(parse_register))),
        |(a, b, c)| [a, b, c],
    )(input)
}

fn parse_program(input: &str) -> IResult<&'_ str, Vec<u8>> {
    preceded(tag("Program: "), separated_list1(char(','), u8))(input)
}

fn parse_input(input: &str) -> ([i64; 3], Vec<u8>) {
    let result: IResult<_, _> = tuple((parse_registers, parse_program))(input);

    result.expect("👊").1
}

impl Op {
    fn takes_combo(&self) -> bool {
        matches!(self, Op::Adv | Op::Bdv | Op::Cdv | Op::Bst | Op::Out)
    }

    fn apply(
        &self,
        operand: i64,
        state: &mut State,
    ) -> Result<Option<i64>, ()> {
        let result = match self {
            Op::Adv => {
                debug_assert!(operand >= 0);
                debug_assert!(operand < 63);
                state.abc[0] /= 2_i64.pow(operand as u32);
                None
            }
            Op::Bdv => {
                debug_assert!(operand >= 0);
                debug_assert!(operand < 63);
                state.abc[1] = state.abc[0] / 2_i64.pow(operand as u32);
                None
            }
            Op::Cdv => {
                debug_assert!(operand >= 0);
                debug_assert!(operand < 63);
                state.abc[2] = state.abc[0] / 2_i64.pow(operand as u32);
                None
            }
            Op::Bxl => {
                state.abc[1] ^= operand;
                None
            }
            Op::Bst => {
                state.abc[1] = operand % 8;
                None
            }
            Op::Jnz => {
                if state.abc[0] > 0 {
                    state.i = operand.try_into().map_err(|_| {})?;
                    debug_assert!(state.i % 2 == 0);
                }

                None
            }
            Op::Bxc => {
                state.abc[1] ^= state.abc[2];
                None
            }
            Op::Out => Some(operand % 8),
        };

        Ok(result)
    }
}

impl<'a> State<'a> {
    fn next_pair(&mut self) -> Result<(Op, i64), ()> {
        let op = Op::from_repr(self.p.get(self.i as usize).copied().ok_or(())?)
            .unwrap();
        let operand_val = self.p.get(self.i as usize + 1).copied().ok_or(())?;

        self.i += 2;

        let operand = if !op.takes_combo() || operand_val < 4 {
            operand_val as i64
        } else {
            self.abc[operand_val as usize - 4]
        };

        Ok((op, operand))
    }

    fn step(&mut self) -> Result<Option<i64>, ()> {
        let (op, operand) = self.next_pair()?;
        // println!("{op:?} -> {operand}");
        // println!("Before \n{:#?}", self);

        // let r = op.apply(operand, self);
        // println!("After \n{:#?}", self);
        // r
        op.apply(operand, self)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (abc, p) = parse_input(input);
    let mut state = State {
        abc,
        p: p.as_slice(),
        i: 0,
    };

    let mut output = vec![];

    while let Ok(output_opt) = state.step() {
        output.extend(output_opt.into_iter());
    }

    Some(output.into_iter().map(|v| v.to_string()).join(","))
}

fn check_a(a: i64, p: &[u8]) -> (bool, usize) {
    let mut state = State {
        abc: [a, 0, 0],
        p,
        i: 0,
    };

    let mut output_cursor = 0;

    while let Ok(output_opt) = state.step() {
        if let Some(output) = output_opt {
            if state.p[output_cursor] != output as u8 {
                break;
            }
            output_cursor += 1;
        }
    }

    (output_cursor == state.p.len(), output_cursor)
}

pub fn part_two_test(input: &str) -> Option<i64> {
    let (_, p) = parse_input(input);

    for a in 0.. {
        let (done, dist) = check_a(a, &p);

        if done {
            return Some(a);
        }

        if dist > 8 {
            println!("{dist}\t<- {a:050b} - {a}");
        }
    }

    unreachable!()
}

/// There's nothing intuitive in the code here. Here's what I did.
/// - Start with a full brute force search and have it print out A in binary
///   the match reaches a certain milestone (I started with 9)
/// - Look for patterns in the bits. For me I saw that all milestone-reaching As
///   all ended with one of 3 19-bit strings
/// - Chang the brute force to only test values that end with the bits found
///   above
/// - Up the milestone that As need to reach before printing (I did 13)
/// - Look for patterns in the the bits again. For me I saw that the middle 17
///   bits (after the 19 bits from earlier) are one of 6 possible values
/// - Restrict the brute force again to only test values with a combination of
///   possible middle bits and the the possible end bits
pub fn part_two(input: &str) -> Option<i64> {
    let (_, p) = parse_input(input);

    for a in (0..)
        .map(|i| i << 17)
        .flat_map(|i| {
            [
                i + 0b11010010011011110,
                i + 0b11010011100001101,
                i + 0b11010011111010010,
                i + 0b11010011111010011,
                i + 0b11011011100001101,
                i + 0b11011111100001101,
            ]
        })
        .map(|i| i << 19)
        .flat_map(|i| {
            [
                i + 0b10101000101010,
                i + 0b10101000101101,
                i + 0b10101000101111,
            ]
        })
    {
        let (done, _dist) = check_a(a, &p);

        if done {
            return Some(a);
        }

        // if dist > 13 {
        //     println!("{dist}\t<- {a:050b} - {a}");
        // }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_test(
            &advent_of_code::template::read_extra_example_file(DAY, 2),
        );
        assert_eq!(result, Some(117440));
    }
}
