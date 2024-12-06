use std::collections::HashSet;

use advent_of_code::{Compass, Grid};

advent_of_code::solution!(6);

fn get_seen_positions(start: usize, grid: &Grid<char>) -> HashSet<usize> {
    let mut seen = HashSet::new();
    let mut d = Compass::N;
    let mut p = start;

    seen.insert(p);

    while let Some(next) = grid.step_from_index(p, d) {
        if grid.data[next] == '#' {
            d = d.turn_right();
        } else {
            p = next;
            seen.insert(p);
        }
    }

    seen
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse_lines(input);

    let start = grid
        .data
        .iter()
        .enumerate()
        .find_map(|(i, c)| (*c == '^').then_some(i))
        .unwrap();

    Some(get_seen_positions(start, &grid).len())
}

fn has_cycle(start: usize, grid: &Grid<char>) -> bool {
    let mut d = Compass::N;
    let mut p = start;

    let mut seen = HashSet::new();
    seen.insert((p, d));

    while let Some(next) = grid.step_from_index(p, d) {
        if grid.data[next] == '#' {
            d = d.turn_right();
        } else {
            p = next;
            if !seen.insert((p, d)) {
                return true;
            }
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Grid<char> = Grid::parse_lines(input);
    let start = grid
        .data
        .iter()
        .enumerate()
        .find_map(|(i, c)| (*c == '^').then_some(i))
        .unwrap();

    let possible_positions = get_seen_positions(start, &grid);

    let result = possible_positions
        .into_iter()
        .filter(|ob| {
            grid.data[*ob] = '#';

            let result = has_cycle(start, &grid);

            grid.data[*ob] = '.';

            result
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
