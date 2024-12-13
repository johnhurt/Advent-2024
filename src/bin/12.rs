use std::{cell::RefCell, collections::HashSet};

use advent_of_code::{Compass, Grid};
use strum::IntoEnumIterator;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Edge {
    col: usize,
    row: usize,
    dir: Compass,
}

impl Edge {
    fn from_cell(grid: &Grid<char>, n: usize, edge_dir: Compass) -> Self {
        let (col, row) = grid.to_col_row(n);
        match edge_dir {
            Compass::N => Edge {
                col,
                row,
                dir: Compass::E,
            },
            Compass::E => Edge {
                col: col + 1,
                row,
                dir: Compass::S,
            },
            Compass::S => Edge {
                col,
                row: row + 1,
                dir: Compass::E,
            },
            Compass::W => Edge {
                col,
                row,
                dir: Compass::S,
            },
        }
    }

    fn sanitize(&self) -> Self {
        let mut result = *self;

        match self.dir {
            Compass::N => {
                result.row -= 1;
                result.dir = Compass::S;
            }
            Compass::W => {
                result.col -= 1;
                result.dir = Compass::E;
            }
            _ => {}
        }

        result
    }

    fn next_directions(&self) -> [Compass; 3] {
        [self.dir.turn_right(), self.dir.turn_left(), self.dir]
    }

    fn next(&self, dir: Compass) -> Option<Self> {
        use Compass as C;
        let mut result = *self;

        debug_assert!(!matches!(
            (self.col, self.row, self.dir),
            (0, _, C::W) | (_, 0, C::N)
        ));

        match self.dir {
            Compass::E => result.col += 1,
            Compass::W => result.col -= 1,
            Compass::S => result.row += 1,
            Compass::N => result.row -= 1,
        }

        result.dir = dir;

        if matches!(
            (self.col, self.row, dir),
            (0, _, Compass::W) | (_, 0, Compass::N)
        ) {
            None
        } else {
            Some(result)
        }
    }
}

fn geometry(
    grid: &Grid<char>,
    c: char,
    start: usize,
    nodes: &mut HashSet<usize>,
    edges: &mut HashSet<Edge>,
) {
    edges.clear();
    nodes.clear();
    let mut search = vec![start];

    while let Some(curr) = search.pop() {
        if !nodes.insert(curr) {
            continue;
        }

        Compass::iter().for_each(|d| {
            edges.insert(Edge::from_cell(grid, curr, d));
        });

        for (d, n) in grid.neighbors(curr).filter(|(_, n)| (grid.data[*n] == c))
        {
            let e = Edge::from_cell(grid, curr, d);
            if nodes.contains(&n) {
                edges.remove(&e);
            } else {
                search.push(n);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse_lines(input);
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    let seen = RefCell::new(HashSet::new());

    let result = grid
        .data
        .iter()
        .enumerate()
        .filter(|(i, _)| !seen.borrow().contains(i))
        .map(|(i, n)| {
            geometry(&grid, *n, i, &mut nodes, &mut edges);
            seen.borrow_mut().extend(nodes.iter().copied());
            edges.len() * nodes.len()
        })
        .sum();

    Some(result)
}

fn count_sides_in_edges(edges: &mut HashSet<Edge>) -> usize {
    let mut total_sides = 0;

    while !edges.is_empty() {
        let mut curr = *edges.iter().next().unwrap();
        let mut sides = 0;

        while let Some(next) = curr
            .next_directions()
            .into_iter()
            .filter_map(|dir| curr.next(dir))
            .find(|e| edges.contains(&e.sanitize()))
        {
            if sides > 0 {
                edges.remove(&curr.sanitize());
            }

            if next.dir != curr.dir {
                sides += 1;
            }

            curr = next;
        }

        total_sides += sides;
        edges.remove(&curr);
    }

    total_sides
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse_lines(input);
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    let seen = RefCell::new(HashSet::new());

    let result = grid
        .data
        .iter()
        .enumerate()
        .filter(|(i, _)| !seen.borrow().contains(i))
        .map(|(i, n)| {
            geometry(&grid, *n, i, &mut nodes, &mut edges);
            seen.borrow_mut().extend(nodes.iter().copied());
            let sides = count_sides_in_edges(&mut edges);
            sides * nodes.len()
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let r = part_one("AAAA\nBBCD\nBBCC\nEEEC").unwrap();
        assert_eq!(r, 140)
    }

    #[test]
    fn test_2() {
        let r = part_one("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO").unwrap();
        assert_eq!(r, (25 - 4) * 36 + 4 * 4)
    }

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_3() {
        let r = part_two("AAAA\nABAA\nAAAA\nAAAA").unwrap();
        assert_eq!(r, 15 * (4 + 4) + 4)
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206 - 224 + 13 * 20 + 4));
    }
}
