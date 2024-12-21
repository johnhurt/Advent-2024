use std::collections::VecDeque;

use advent_of_code::Grid;
use itertools::Itertools;
use tinyvec::TinyVec;

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy)]
enum CheatMapEntry {
    Path(usize),
    Wall,
}

impl From<char> for CheatMapEntry {
    fn from(value: char) -> Self {
        match value {
            'E' => CheatMapEntry::Path(0),
            '#' => CheatMapEntry::Wall,
            _ => CheatMapEntry::Path(usize::MAX),
        }
    }
}

fn fill_nominal_distances(grid: &mut Grid<CheatMapEntry>) {
    use CheatMapEntry as E;
    let end = grid
        .data
        .iter()
        .enumerate()
        .find_map(|(i, e)| matches!(e, E::Path(0)).then_some(i))
        .expect("🥶");

    let mut queue = [(end, 0)].into_iter().collect::<VecDeque<_>>();

    while let Some((curr, dist)) = queue.pop_front() {
        grid.data[curr] = E::Path(dist);
        queue.extend(
            grid.neighbors(curr)
                .filter(|(_, n)| matches!(&grid.data[*n], E::Path(usize::MAX)))
                .map(|(_, n)| (n, dist + 1)),
        )
    }
}

fn p1_limited(input: &str, minimum_savings: usize) -> usize {
    use CheatMapEntry as E;
    let mut grid: Grid<CheatMapEntry> = Grid::parse_lines(input);

    fill_nominal_distances(&mut grid);

    grid.data
        .iter()
        .enumerate()
        .filter(|(_, e)| matches!(e, E::Wall))
        .filter_map(|(i, _)| {
            let distances = grid
                .neighbors(i)
                .filter_map(|(_, n)| {
                    if let E::Path(d) = &grid.data[n] {
                        Some(*d)
                    } else {
                        None
                    }
                })
                .collect::<TinyVec<[usize; 4]>>();

            distances
                .iter()
                .copied()
                .cartesian_product(distances.iter().copied())
                .map(|(f, t)| f.abs_diff(t).saturating_sub(1))
                .max()
        })
        .filter(|d| *d >= minimum_savings)
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(p1_limited(input, 100))
}

fn p2_limited(input: &str, minimum_savings: usize) -> usize {
    use CheatMapEntry as E;
    let mut grid: Grid<CheatMapEntry> = Grid::parse_lines(input);

    fill_nominal_distances(&mut grid);

    (0..grid.data.len())
        .cartesian_product(0..grid.data.len())
        .filter(|(i, j)| i < j)
        .filter(|(i, j)| grid.min_dist(*i, *j) <= 20)
        .filter_map(|(i, j)| match (&grid.data[i], &grid.data[j]) {
            (E::Path(di), E::Path(dj)) => {
                Some(di.abs_diff(*dj).saturating_sub(grid.min_dist(i, j) - 1))
            }
            _ => None,
        })
        .filter(|d| *d >= minimum_savings)
        .count()
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(p2_limited(input, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_4() {
        let result = p1_limited(
            &advent_of_code::template::read_file("examples", DAY),
            4,
        );
        assert_eq!(result, 30);
    }

    #[test]
    fn test_part_one_20() {
        let result = p1_limited(
            &advent_of_code::template::read_file("examples", DAY),
            20,
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_two_50() {
        let result = p2_limited(
            &advent_of_code::template::read_file("examples", DAY),
            50,
        );
        assert_eq!(result, 285);
    }

    #[test]
    fn test_part_two_72() {
        let result = p2_limited(
            &advent_of_code::template::read_file("examples", DAY),
            72,
        );
        assert_eq!(result, 29);
    }
}
