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

fn has_cycle(
    start: usize,
    grid: &Grid<char>,
    working_space: &mut [[u16; 4]],
    generation: u16,
) -> bool {
    let mut d = Compass::N;
    let mut p = start;

    working_space[p][d as u8 as usize] = generation;

    while let Some(next) = grid.step_from_index(p, d) {
        if grid.data[next] == '#' {
            d = d.turn_right();
        } else {
            p = next;

            let cell = &mut working_space[p][d as u8 as usize];

            if *cell == generation {
                return true;
            }

            *cell = generation;
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

    // Originally I detected cycles with a hashmap, but that was very slow.
    // This method uses a fixed (but large) amount of memory for all checks of
    // cycles. We duplicate the shape of the grid with a slot for each direction
    // the guard could be facing there.
    //
    // For each possible position of the new obstacle, we set a new "generation"
    // and use that as the marker for position-direction pairs, if a traversal
    // of the new grid encounters a slot with the current generation, it
    // indicates a repeated vector and a cycle
    let mut cycle_detector = vec![[0; 4]; grid.data.len()];

    let result = possible_positions
        .into_iter()
        .enumerate()
        .filter(|(i, ob)| {
            grid.data[*ob] = '#';

            let result =
                has_cycle(start, &grid, &mut cycle_detector, *i as u16 + 1);

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
