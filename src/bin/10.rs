use std::collections::HashSet;

use advent_of_code::Grid;

advent_of_code::solution!(10);

fn score_trail_head(grid: &Grid<u8>, start: usize) -> (usize, usize) {
    let mut to_search = vec![(start, 1)];
    let mut total = 0;
    let mut reached = HashSet::new();

    while let Some((curr, needs)) = to_search.pop() {
        if needs == 10 {
            total += 1;
            reached.insert(curr);
            continue;
        }

        to_search.extend(
            grid.neighbors(curr)
                .filter(|(_, n)| grid.data[*n] == needs)
                .map(|(_, n)| (n, needs + 1)),
        );
    }

    (reached.len(), total)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<u8> = Grid::parse_lines(input).map(|c: char| c as u8 - b'0');

    let result = (0..grid.data.len())
        .filter(|i| grid.data[*i] == 0)
        .map(|start| score_trail_head(&grid, start).0)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<u8> = Grid::parse_lines(input).map(|c: char| c as u8 - b'0');

    let result = (0..grid.data.len())
        .filter(|i| grid.data[*i] == 0)
        .map(|start| score_trail_head(&grid, start).1)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
