use std::collections::{HashSet, VecDeque};

use advent_of_code::{ws, Grid};
use nom::{
    character::complete::{char, u64},
    combinator::map,
    multi::many1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(18);

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let result: IResult<_, _> =
        many1(ws(map(separated_pair(u64, char(','), u64), |(c, r)| {
            (c as usize, r as usize)
        })))(input);

    result.expect("🤫").1
}

fn bfs(grid: &Grid<char>, start: usize, end: usize) -> Option<usize> {
    let mut queue = [(start, 0)].into_iter().collect::<VecDeque<_>>();
    let mut seen = HashSet::new();

    while let Some((curr, dist)) = queue.pop_front() {
        if curr == end {
            return Some(dist);
        }

        queue.extend(
            grid.neighbors(curr)
                .filter_map(|(_, n)| {
                    (grid.data[n] != '#').then_some((n, dist + 1))
                })
                .filter(|(n, _)| seen.insert(*n)),
        );
    }

    None
}

fn p1_sized(
    input: &str,
    size: (usize, usize),
    fall_count: usize,
) -> Option<usize> {
    let mut grid: Grid<char> = Grid::new(vec!['.'; size.0 * size.1], size.0);
    let bytes = parse_input(input);

    bytes.into_iter().take(fall_count).for_each(|(c, r)| {
        let i = grid.to_index(c, r);
        grid.data[i] = '#';
    });

    bfs(&grid, 0, grid.data.len() - 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    p1_sized(input, (71, 71), 1024)
}

fn binary_search(grid: &mut Grid<char>, bytes: &[(usize, usize)]) -> usize {
    let mut left = 0;
    let mut right = bytes.len();
    let start = 0;
    let end = grid.data.len() - 1;

    while right - left > 1 {
        let middle = (right + left) / 2;

        bytes.iter().copied().take(middle).for_each(|(c, r)| {
            let i = grid.to_index(c, r);
            debug_assert!(grid.data[i] == '.');
            grid.data[i] = '#';
        });

        if bfs(grid, start, end).is_some() {
            left = middle;
        } else {
            right = middle;
        }

        bytes.iter().copied().take(middle).for_each(|(c, r)| {
            let i = grid.to_index(c, r);
            debug_assert!(grid.data[i] == '#');
            grid.data[i] = '.';
        });
    }

    left
}

fn p2_sized(input: &str, size: (usize, usize), start_fall: usize) -> String {
    let mut grid: Grid<char> = Grid::new(vec!['.'; size.0 * size.1], size.0);
    let bytes = parse_input(input);

    let mut fall_count = start_fall + 1;

    bytes.iter().copied().take(fall_count).for_each(|(c, r)| {
        let i = grid.to_index(c, r);
        grid.data[i] = '#';
    });

    fall_count += binary_search(&mut grid, &bytes[fall_count..]);

    format!("{},{}", bytes[fall_count].0, bytes[fall_count].1)
}

pub fn part_two(input: &str) -> Option<String> {
    Some(p2_sized(input, (71, 71), 1024))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = p1_sized(
            &advent_of_code::template::read_file("examples", DAY),
            (7, 7),
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = p2_sized(
            &advent_of_code::template::read_file("examples", DAY),
            (7, 7),
            12,
        );
        assert_eq!(result, "6,1".to_owned());
    }
}
