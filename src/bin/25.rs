use advent_of_code::{Compass, Grid};
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::terminated};

advent_of_code::solution!(25);

#[derive(Debug, Clone, Copy)]
enum V {
    L([usize; 5]),
    K([usize; 5]),
}

impl V {
    fn from_grid(grid: Grid<char>) -> V {
        if grid.ray(0, Compass::E.into()).all(|(_, c)| *c == '#') {
            let mut k = [0; 5];
            (0..5).for_each(|i| {
                k[i] = grid
                    .ray(i, Compass::S.into())
                    .take_while(|(_, c)| **c == '#')
                    .count()
                    - 1
            });
            V::K(k)
        } else {
            let mut l = [0; 5];
            (0..5)
                .map(|i| (i, grid.data.len() - grid.width + i))
                .for_each(|(i, j)| {
                    l[i] = grid
                        .ray(j, Compass::N.into())
                        .take_while(|(_, c)| **c == '#')
                        .count()
                        - 1
                });
            V::L(l)
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut h = None;

    let vs = input
        .split("\n\n")
        .map(Grid::parse_lines)
        .inspect(|g| {
            if let Some(hs) = h {
                assert_eq!(g.height, hs);
            } else {
                h = Some(g.height);
            }
        })
        .map(V::from_grid)
        .collect_vec();

    let ks = vs.iter().copied().filter_map(|v| {
        if let V::K(k) = v {
            Some(k)
        } else {
            None
        }
    });

    let ls = vs.iter().copied().filter_map(|v| {
        if let V::L(l) = v {
            Some(l)
        } else {
            None
        }
    });

    let result = ks
        .into_iter()
        .cartesian_product(ls.into_iter())
        .filter(|(k, l)| {
            k.iter()
                .copied()
                .zip(l.iter().copied())
                .all(|(l, r)| l + r < h.unwrap() - 1)
        })
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
