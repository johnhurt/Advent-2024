use std::collections::{HashMap, HashSet};

use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(8);

fn record_first_antinode_locations(
    antennas: &[(i64, i64)],
    record: &mut HashSet<(i64, i64)>,
) {
    record.extend(
        antennas
            .iter()
            .cartesian_product(antennas)
            .filter(|(l, r)| l != r)
            .flat_map(|(l, r)| {
                let dx = l.0 - r.0;
                let dy = l.1 - r.1;

                [(l.0 + dx, l.1 + dy), (r.0 - dx, r.1 - dy)]
            }),
    );
}

fn get_antennas(grid: &Grid<char>) -> HashMap<char, Vec<(i64, i64)>> {
    let to_signed_col_row = |i| {
        let (col, row) = grid.to_col_row(i);
        (col as i64, row as i64)
    };

    grid.data
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, c)| (c != '.').then_some((c, i)))
        .map(|(c, i)| (c, to_signed_col_row(i)))
        .into_group_map()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse_lines(input);

    let antennas = get_antennas(&grid);
    let mut antinodes = HashSet::new();

    antennas.into_values().for_each(|locations| {
        record_first_antinode_locations(&locations, &mut antinodes)
    });

    let result = antinodes
        .into_iter()
        .filter(|(c, r)| *c >= 0 && *r >= 0)
        .filter(|(c, r)| {
            (*c as usize) < grid.width && (*r as usize) < grid.height
        })
        .count();

    Some(result)
}

fn record_all_antinode_locations(
    antennas: &[(i64, i64)],
    max_col: i64,
    max_row: i64,
    record: &mut HashSet<(i64, i64)>,
) {
    record.extend(
        antennas
            .iter()
            .cartesian_product(antennas)
            .filter(|(l, r)| l != r)
            .flat_map(|(left, right)| {
                let dx = left.0 - right.0;
                let dy = left.1 - right.1;

                [(left, (dx, dy)), (left, (-dx, -dy))]
            })
            .flat_map(|(left, d)| {
                (0..)
                    .map(move |i| (left.0 + d.0 * i, left.1 + d.1 * i))
                    .take_while(|(col, row)| {
                        *col >= 0
                            && *row >= 0
                            && *col <= max_col
                            && *row <= max_row
                    })
            }),
    );
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse_lines(input);

    let antennas = get_antennas(&grid);
    let mut antinodes = HashSet::new();

    antennas.into_values().for_each(|locations| {
        record_all_antinode_locations(
            &locations,
            grid.width as i64 - 1,
            grid.height as i64 - 1,
            &mut antinodes,
        )
    });

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
