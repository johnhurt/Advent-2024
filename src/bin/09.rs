use itertools::Itertools;

advent_of_code::solution!(9);

struct FragMapBlockIter<'a> {
    frag_map: &'a str,
    map_index: usize,
    chunk: Option<usize>,
    blocks_remaining: usize,
    rev: bool,
}

impl<'a> FragMapBlockIter<'a> {
    fn new(frag_map: &'a str, rev: bool) -> Self {
        let map_index = if rev { frag_map.len() - 1 } else { 0 };
        let mut result = FragMapBlockIter {
            frag_map,
            map_index,
            blocks_remaining: 0,
            chunk: None,
            rev,
        };

        result.update_map_index(map_index);

        result
    }

    fn update_map_index(&mut self, map_index: usize) {
        self.map_index = map_index;
        self.chunk = (map_index % 2 == 0).then_some(map_index / 2);
        self.blocks_remaining =
            (self.frag_map.as_bytes()[map_index] - b'0') as usize;
    }
}

impl<'a> Iterator for FragMapBlockIter<'a> {
    type Item = Option<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.blocks_remaining == 0 {
            let next_index = if self.rev {
                self.map_index.checked_sub(1)?
            } else {
                self.map_index
                    .checked_add(1)
                    .filter(|i| *i < self.frag_map.len())?
            };

            self.update_map_index(next_index);
        }

        self.blocks_remaining -= 1;
        Some(self.chunk)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let total_chunks = input
        .bytes()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 0)
        .map(|(_, c)| (c - b'0') as usize)
        .sum();
    let forward = FragMapBlockIter::new(input, false);
    let mut backward = FragMapBlockIter::new(input, true).flatten();

    let result = forward
        .take(total_chunks)
        .enumerate()
        .map(|(i, block_opt)| {
            i * block_opt.or_else(|| backward.next()).unwrap()
        })
        .sum();

    Some(result)
}

#[derive(Debug)]
struct FsNode {
    index: Option<usize>,
    len: usize,
    next: Option<usize>,
    prev: Option<usize>,
}

fn sum_between(left: usize, right_ex: usize) -> usize {
    let left = left.saturating_sub(1);
    let right_ex = right_ex.saturating_sub(1);
    right_ex * (right_ex + 1) / 2 - left * (left + 1) / 2
}

impl FsNode {
    fn check_sum(&self, start: usize) -> usize {
        let Some(index) = self.index else {
            return 0;
        };

        sum_between(start, start + self.len) * index
    }
}

struct FsList {
    nodes: Vec<FsNode>,
}

impl FsList {
    fn new(input: &str) -> Self {
        let nodes = input
            .bytes()
            .enumerate()
            .map(|(i, b)| FsNode {
                index: (i % 2 == 0).then_some(i / 2),
                len: (b - b'0') as usize,
                prev: i.checked_sub(1),
                next: i.checked_add(1).filter(|n| *n < input.len()),
            })
            .collect_vec();

        FsList { nodes }
    }

    fn find_first_slot(&self, size: usize, limit: usize) -> Option<usize> {
        let mut node_index = 0;

        loop {
            if node_index == limit {
                return None;
            }

            let node = &self.nodes[node_index];
            if node.index.is_none() && node.len >= size {
                return Some(node_index);
            }

            let next = node.next?;

            node_index = next;
        }
    }

    fn remove_node(&mut self, node: usize) {
        let left_opt = self.nodes[node].prev;
        let right_opt = self.nodes[node].next;

        let left = match (left_opt, right_opt) {
            (Some(left), Some(right)) => {
                self.nodes[left].next = Some(right);
                self.nodes[right].prev = Some(left);
                left
            }
            (Some(left), None) => {
                self.nodes[left].next = None;
                left
            }
            _ => unreachable!(),
        };

        self.nodes[left].len += self.nodes[node].len;
    }

    fn insert_before(&mut self, to_insert: usize, before: usize) {
        let left = self.nodes[before].prev.unwrap();
        self.nodes[left].next = Some(to_insert);
        self.nodes[to_insert].prev = Some(left);
        self.nodes[before].prev = Some(to_insert);
        self.nodes[to_insert].next = Some(before);
        self.nodes[before].len -= self.nodes[to_insert].len;
    }

    fn move_chunk(&mut self, from: usize, to: usize) {
        self.remove_node(from);
        self.insert_before(from, to);
    }

    fn check_sum(&self) -> usize {
        let mut node_opt = Some(0);
        let mut result = 0;
        let mut start = 0;

        while let Some(node) = node_opt {
            result += self.nodes[node].check_sum(start);
            start += self.nodes[node].len;
            node_opt = self.nodes[node].next;
        }

        result
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut list = FsList::new(input);

    let moveable = (0..list.nodes.len()).rev().filter(|i| i % 2 == 0);

    for to_move in moveable {
        let node = &list.nodes[to_move];
        if let Some(target) = list.find_first_slot(node.len, to_move) {
            list.move_chunk(to_move, target);
        }
    }

    Some(list.check_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
