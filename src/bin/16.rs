use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use advent_of_code::{Compass, Grid};
use priority_queue::PriorityQueue;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<i64> {
    use Compass as D;
    let grid: Grid<char> = Grid::parse_lines(input);

    let mut start = 0;
    let mut end = 0;

    grid.data.iter().enumerate().for_each(|(i, v)| {
        if *v == 'S' {
            start = i;
        } else if *v == 'E' {
            end = i;
        }
    });

    let mut pq = PriorityQueue::new();
    pq.push((start, D::E), 0);
    let mut seen = HashSet::new();
    seen.insert((start, D::E));

    while let Some(((curr_p, curr_d), dist)) = pq.pop() {
        if curr_p == end {
            return Some(-dist);
        }

        let dir_l = curr_d.turn_left();
        let dir_r = curr_d.turn_right();

        let next_l = grid.step_from_index(curr_p, dir_l).unwrap();
        if grid.data[next_l] != '#' && seen.insert((curr_p, dir_l)) {
            pq.push((curr_p, dir_l), dist - 1000);
        }

        let next_r = grid.step_from_index(curr_p, dir_r).unwrap();
        if grid.data[next_r] != '#' && seen.insert((curr_p, dir_r)) {
            pq.push((curr_p, dir_r), dist - 1000);
        }

        let next_p = grid.step_from_index(curr_p, curr_d).unwrap();
        if grid.data[next_p] != '#' && seen.insert((next_p, curr_d)) {
            pq.push((next_p, curr_d), dist - 1);
        }
    }

    None
}

#[derive(Debug)]
struct Path {
    seen: HashSet<(usize, Compass)>,
    curr: (usize, Compass),
}

impl Path {
    fn new(
        mut seen: HashSet<(usize, Compass)>,
        curr: (usize, Compass),
    ) -> Self {
        seen.insert(curr);
        Path { seen, curr }
    }
}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.curr.hash(state);
    }
}

impl PartialEq for Path {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for Path {}

pub fn part_two(input: &str) -> Option<usize> {
    use Compass as D;
    let grid: Grid<char> = Grid::parse_lines(input);
    let mut fastest = HashMap::new();

    let mut start = 0;
    let mut end = 0;

    grid.data.iter().enumerate().for_each(|(i, v)| {
        if *v == 'S' {
            start = i;
        } else if *v == 'E' {
            end = i;
        }
    });

    let mut pq = PriorityQueue::new();
    pq.push(Path::new(HashSet::new(), (start, D::E)), 0);

    let mut next_list = Vec::with_capacity(3);

    let mut shortest_opt = None;
    let mut nodes_in_any_path = HashSet::new();
    let mut used_paths = Vec::new();

    while let Some((path, dist)) = pq.pop() {
        let Path {
            mut seen,
            curr: (curr_p, curr_d),
        } = path;

        if curr_p == end {
            if let Some(shortest) = shortest_opt {
                if dist != shortest {
                    break;
                }
            } else {
                shortest_opt = Some(dist);
            }

            // let mut g = grid.clone();
            // seen.iter().for_each(|(i, _)| g.data[*i] = 'O');
            // g.print();

            nodes_in_any_path.extend(seen.into_iter().map(|(i, _)| i));
            continue;
        }

        if let Some(fastest_d) = fastest.get(&(curr_p, curr_d)) {
            if dist != *fastest_d {
                continue;
            }
        } else {
            fastest.insert((curr_p, curr_d), dist);
        }

        debug_assert!(next_list.is_empty());

        let dir_l = curr_d.turn_left();
        let dir_r = curr_d.turn_right();

        let next_l = grid.step_from_index(curr_p, dir_l).unwrap();
        if grid.data[next_l] != '#' && !seen.contains(&(curr_p, dir_l)) {
            next_list.push(((curr_p, dir_l), dist - 1000));
        }

        let next_r = grid.step_from_index(curr_p, dir_r).unwrap();
        if grid.data[next_r] != '#' && !seen.contains(&(curr_p, dir_r)) {
            next_list.push(((curr_p, dir_r), dist - 1000));
        }

        let next_p = grid.step_from_index(curr_p, curr_d).unwrap();
        if grid.data[next_p] != '#' && !seen.contains(&(next_p, curr_d)) {
            next_list.push(((next_p, curr_d), dist - 1));
        }

        if next_list.is_empty() {
            seen.clear();
            used_paths.push(seen);
        } else {
            while next_list.len() > 1 {
                let (curr, dist) = next_list.pop().unwrap();

                let next_seen = used_paths
                    .pop()
                    .map(|mut used| {
                        used.clone_from(&seen);
                        used
                    })
                    .unwrap_or_else(|| seen.clone());

                let path = Path::new(next_seen, curr);
                pq.push(path, dist);
            }

            let (curr, dist) = next_list.pop().unwrap();
            let path = Path::new(seen, curr);
            pq.push(path, dist);
        }
    }

    Some(nodes_in_any_path.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
