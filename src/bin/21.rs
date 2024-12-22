use std::collections::VecDeque;

use advent_of_code::Grid;
use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::RngCore;
use strum::{Display, EnumIter, EnumString, FromRepr};

advent_of_code::solution!(21);

/// The grid for part 1 is
///    +---+---+---+
///    | 7 | 8 | 9 |
///    +---+---+---+
///    | 4 | 5 | 6 |
///    +---+---+---+
///    | 1 | 2 | 3 |
///    +---+---+---+
///        |0/^| A |
///    +---+---+---+
///    | < | v | > |
///    +---+---+---+
/// 7 is at index 0 and index 9 is not accessible

const BUTTON_COUNT: usize = 15;

static GRID: Lazy<Grid<Button>> = Lazy::new(|| {
    Grid::new(
        (0..BUTTON_COUNT)
            .map(|i| Button::from_repr(i as u8).unwrap())
            .collect_vec(),
        3,
    )
});

#[derive(
    EnumString,
    FromRepr,
    Display,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    EnumIter,
    Hash,
)]
#[repr(u8)]
enum Button {
    Seven,
    Eight,
    Nine,
    Four,
    Five,
    Six,
    One,
    Two,
    Three,
    None,
    ZeroUp,
    #[default]
    A,
    Left,
    Down,
    Right,
}

impl From<u8> for Button {
    fn from(c: u8) -> Self {
        match c {
            b'0' => Button::ZeroUp,
            b'1' => Button::One,
            b'2' => Button::Two,
            b'3' => Button::Three,
            b'4' => Button::Four,
            b'5' => Button::Five,
            b'6' => Button::Six,
            b'7' => Button::Seven,
            b'8' => Button::Eight,
            b'9' => Button::Nine,
            b'A' => Button::A,
            b'>' => Button::Right,
            b'<' => Button::Left,
            b'^' => Button::ZeroUp,
            b'v' => Button::Down,
            _ => unreachable!(),
        }
    }
}

impl Button {
    fn same_class(b1: usize, b2: usize) -> bool {
        matches!((b1, b2), (0..=8 | 10 | 11, 0..=8 | 10 | 11) | (10.., 10..))
    }
}

type Path = Vec<Button>;

fn seeded_all_paths(mut seed: u64) -> Vec<Path> {
    (0..(BUTTON_COUNT.pow(2)))
        .map(|i| (i / BUTTON_COUNT, i % BUTTON_COUNT))
        .map(|(start, end)| shortest_path(start, end, &mut seed))
        .collect_vec()
}

fn count_changes(path: &Path) -> usize {
    path.iter()
        .fold((0, None), |(acc, prev), curr| {
            if let Some(prev) = prev {
                (acc + (curr != prev) as usize, Some(curr))
            } else {
                (0, Some(curr))
            }
        })
        .0
}

fn shortest_path(start: usize, end: usize, seed: &mut u64) -> Path {
    let mut result = vec![];

    if !Button::same_class(start, end) {
        return vec![];
    }

    let mut queue = [(start, 1 << start, Path::default(), 0)]
        .into_iter()
        .collect::<VecDeque<_>>();
    let mut shortest_opt = None;

    while let Some((curr, seen, path, dist)) = queue.pop_front() {
        if curr == end {
            if let Some(shortest) = shortest_opt {
                if shortest < dist {
                    break;
                }
            } else {
                shortest_opt = Some(dist);
            }

            if count_changes(&path) < 2 {
                result.push(path);
            }
            continue;
        }

        queue.extend(
            GRID.neighbors(curr)
                .filter(|(_, n)| GRID.data[*n] != Button::None)
                .map(|(d, n)| (d, n, (1 << n) | seen))
                .filter(|(_, _, n_seen)| *n_seen != seen)
                .map(|(d, n, n_seen)| {
                    let mut n_path = path.clone();

                    n_path.push(d.to_arrow().into());
                    (n, n_seen, n_path, dist + 1)
                }),
        );
    }

    result.iter_mut().for_each(|p| p.push(Button::A));

    if result.len() > 1 {
        *seed /= 2;
        if *seed % 2 == 1 {
            result.into_iter().nth(1).unwrap()
        } else {
            result.into_iter().next().unwrap()
        }
    } else {
        result.into_iter().next().unwrap()
    }
}

fn parse_buttons(line: &str) -> (usize, [Button; 4]) {
    let value: usize = line[..3].parse().unwrap();

    let mut result = [Button::A; 4];

    result
        .iter_mut()
        .take(3)
        .enumerate()
        .for_each(|(i, t)| *t = line.as_bytes()[i].into());

    (value, result)
}

fn fast_single_answer(
    buttons: &[Button],
    iterations: usize,
    all_paths: &[Path],
) -> usize {
    let mut pairs = vec![0; BUTTON_COUNT * BUTTON_COUNT];

    let pair_contributions = all_paths
        .iter()
        .map(|path| {
            if path.is_empty() {
                vec![]
            } else {
                Some(Button::A)
                    .into_iter()
                    .chain(path.iter().copied())
                    .tuple_windows::<(_, _)>()
                    .map(|(f, t)| f as usize * BUTTON_COUNT + t as usize)
                    .collect_vec()
            }
        })
        .collect_vec();

    Some(Button::A)
        .into_iter()
        .chain(buttons.iter().copied())
        .tuple_windows::<(_, _)>()
        .map(|(f, t)| f as usize * BUTTON_COUNT + t as usize)
        .for_each(|i| pairs[i] += 1);

    for _ in 0..iterations {
        let mut new_pairs = vec![0; BUTTON_COUNT * BUTTON_COUNT];

        pairs.drain(..).enumerate().for_each(|(i, count)| {
            pair_contributions[i]
                .iter()
                .for_each(|j| new_pairs[*j] += count)
        });

        pairs.append(&mut new_pairs);
    }

    pairs.into_iter().sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let all_paths = seeded_all_paths(u64::MAX);
    let result = input
        .lines()
        .map(parse_buttons)
        .map(|(value, buttons)| {
            fast_single_answer(&buttons, 3, &all_paths) * value
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    (0..1000)
        .map(|_| {
            let seed = rand::thread_rng().next_u64();
            let all_paths = seeded_all_paths(seed);

            input
                .lines()
                .map(parse_buttons)
                .map(|(value, buttons)| {
                    value * fast_single_answer(&buttons, 26, &all_paths)
                })
                .sum()
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
