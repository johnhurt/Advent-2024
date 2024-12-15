use std::collections::HashSet;

use advent_of_code::{FullCompass, Grid};
use itertools::Itertools;
use strum::IntoEnumIterator;

advent_of_code::solution!(4);

/// Compare an iterator of characters to a string slice
fn iter_eq<I>(left: I, right: &str) -> bool
where
    I: Iterator<Item = char>,
{
    left.take(right.len())
        .zip_longest(right.chars())
        .map(|e| e.left_and_right())
        .all(|(l, r)| l == r)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse_lines(input);

    let result = (0..grid.data.len())
        .cartesian_product(FullCompass::iter())
        .map(|(start, dir)| grid.ray(start, dir))
        .map(|ray| iter_eq(ray.map(|(_, c)| *c), "XMAS"))
        .filter(|v| *v)
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    use FullCompass as D;

    let mut back_as = HashSet::new();
    let mut forwards_as = HashSet::new();

    let grid: Grid<char> = Grid::parse_lines(input);

    (0..grid.data.len())
        .cartesian_product([D::NE, D::NW, D::SE, D::SW])
        .map(|(start, dir)| (start, dir, grid.ray(start, dir)))
        .map(|(start, dir, ray)| {
            (start, dir, iter_eq(ray.map(|(_, c)| *c), "MAS"))
        })
        .filter(|(_, _, keep)| *keep)
        .map(|(start, dir, _)| {
            (
                matches!(dir, D::NW | D::SE),
                grid.step_from_index(start, dir).unwrap(),
            )
        })
        .for_each(|(back, a)| {
            if back {
                back_as.insert(a);
            } else {
                forwards_as.insert(a);
            }
        });

    Some(back_as.intersection(&forwards_as).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
