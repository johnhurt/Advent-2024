use core::panic;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use advent_of_code::ws;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, u8},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, tuple},
    IResult,
};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum G {
    A,
    O,
    X,
}

impl G {
    fn parse(input: &str) -> IResult<&'_ str, Self> {
        map(alphanumeric1, |v| match v {
            "AND" => G::A,
            "OR" => G::O,
            "XOR" => G::X,
            _ => unreachable!(),
        })(input)
    }

    fn eval(&self, l: u8, r: u8) -> u8 {
        match self {
            G::A => l * r,
            G::O => 1.min(l + r),
            G::X => (l + r) % 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct C<N> {
    l: N,
    r: N,
    o: N,
    g: G,
}

impl<N> C<N> {
    fn nodes(&self) -> [&'_ N; 3] {
        [&self.l, &self.r, &self.o]
    }
}

fn parse_node(input: &str) -> IResult<&'_ str, (&'_ str, u8)> {
    separated_pair(ws(alphanumeric1), ws(char(':')), u8)(input)
}

fn parse_edge(input: &str) -> IResult<&'_ str, C<&'_ str>> {
    map(
        separated_pair(
            tuple((alphanumeric1, ws(G::parse), ws(alphanumeric1))),
            ws(tag("->")),
            ws(alphanumeric1),
        ),
        |((l, g, r), o)| C { l, g, r, o },
    )(input)
}

fn parse_input(input: &str) -> (Vec<(&'_ str, u8)>, Vec<C<&'_ str>>) {
    let result: IResult<_, _> =
        separated_pair(many1(parse_node), tag("\n\n"), many1(parse_edge))(
            input,
        );

    result.expect("🪢").1
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nodes, edges) = parse_input(input);
    let mut z_nodes = BTreeMap::new();

    let name_map = nodes
        .iter()
        .map(|(name, _)| name)
        .chain(edges.iter().flat_map(|c| c.nodes()))
        .collect::<HashSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, name)| (*name, i))
        .inspect(|(name, i)| {
            if name.starts_with('z') {
                z_nodes.insert(*name, *i);
            }
        })
        .collect::<HashMap<_, _>>();

    let mut states = vec![None; name_map.len()];

    nodes
        .into_iter()
        .for_each(|(n, v)| states[*name_map.get(n).unwrap()] = Some(v));

    let edges = edges
        .into_iter()
        .map(|C { l, r, o, g }| C {
            l: *name_map.get(l).unwrap(),
            r: *name_map.get(r).unwrap(),
            o: *name_map.get(o).unwrap(),
            g,
        })
        .collect_vec();

    let mut queue = edges.iter().copied().collect::<VecDeque<_>>();

    while let Some(C { l, r, o, g }) = queue.pop_front() {
        match (states[l], states[r]) {
            (Some(n1), Some(n2)) => states[o] = Some(g.eval(n1, n2)),
            _ => queue.push_back(C { l, r, o, g }),
        }
    }

    let result = z_nodes
        .into_values()
        .filter_map(|i| states[i])
        .map(|v| v as u64)
        .enumerate()
        .map(|(i, curr)| (curr << i))
        .sum();

    Some(result)
}

// x3------x---a-x2--a---x--x1---a-----x------y1
//         |   |     |   |       |     |
//         |   o---- m2  |       |     |
//         |   |         |       |     |
//         x --c2        x-------c1    |
//         |             |             |
//         z3            z2            z1
//
pub fn part_two(input: &str) -> Option<String> {
    let (nodes, edges) = parse_input(input);
    let mut x_nodes = BTreeMap::new();
    let mut y_nodes = BTreeMap::new();
    let mut z_nodes = BTreeMap::new();

    let name_map = nodes
        .iter()
        .map(|(name, _)| name)
        .chain(edges.iter().flat_map(|c| c.nodes()))
        .collect::<HashSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, name)| (*name, i))
        .inspect(|(name, i)| {
            match name.as_bytes()[0] {
                b'x' => x_nodes.insert(*name, *i),
                b'y' => y_nodes.insert(*name, *i),
                b'z' => z_nodes.insert(*name, *i),
                _ => None,
            };
        })
        .collect::<HashMap<_, _>>();

    let names = name_map
        .iter()
        .map(|(name, i)| (*i, *name))
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect_vec();

    let x_nodes = x_nodes.into_values().collect_vec();
    let y_nodes = y_nodes.into_values().collect_vec();
    let z_nodes = z_nodes.into_values().collect_vec();

    let edges = edges
        .into_iter()
        .map(|C { l, r, o, g }| C {
            l: *name_map.get(l).unwrap(),
            r: *name_map.get(r).unwrap(),
            o: *name_map.get(o).unwrap(),
            g,
        })
        .collect_vec();

    edges
        .iter()
        .map(|c| (c.g, c))
        .into_group_map()
        .into_iter()
        .for_each(|(k, v)| println!("{k:?} -> {}", v.len()));

    dbg!(x_nodes.len());
    dbg!(y_nodes.len());
    dbg!(z_nodes.len());

    let mut edges_by_output = vec![None; name_map.len()];
    edges.iter().for_each(|c| edges_by_output[c.o] = Some(c));

    let mut has_needs = HashMap::new();

    let wrong_output_gate = z_nodes
        .iter()
        .filter_map(|n| edges_by_output[*n].map(|c| (n, c.g)))
        .filter_map(|(n, g)| {
            if g != G::X {
                has_needs
                    .entry((g, G::X))
                    .and_modify(|v: &mut Vec<_>| v.push(*n))
                    .or_insert(vec![*n]);
                Some(n)
            } else {
                None
            }
        })
        .filter(|n| *n != z_nodes.last().unwrap())
        .copied()
        .collect_vec();

    dbg!(wrong_output_gate
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec());

    let mut edges_by_inputs = vec![vec![]; names.len()];

    edges.iter().copied().for_each(|c| {
        edges_by_inputs[c.l].push(c);
        edges_by_inputs[c.r].push(c);
    });

    let wrong_top_gates = x_nodes
        .iter()
        .copied()
        .chain(y_nodes.iter().copied())
        .map(|n| (n, &edges_by_inputs[n]))
        .filter_map(|(n, cs)| {
            debug_assert_eq!(cs.len(), 2);

            match (cs[0].g, cs[1].g) {
                (G::A, G::X) | (G::X, G::A) => None,
                _ => Some(n),
            }
        })
        .collect_vec();

    debug_assert!(wrong_top_gates
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec()
        .is_empty());

    let mut mismatched_top_inputs = vec![];
    let mut xy_and_nodes = vec![];
    let mut xy_xor_nodes = vec![];

    x_nodes
        .iter()
        .copied()
        .zip(y_nodes.iter().copied())
        .for_each(|(x, y)| {
            let x_and =
                edges_by_inputs[x].iter().find(|c| c.g == G::A).unwrap();

            if x_and.l != y && x_and.r != y {
                mismatched_top_inputs.push(x);
            }

            let x_xor =
                edges_by_inputs[x].iter().find(|c| c.g == G::X).unwrap();

            if x_xor.l != y && x_xor.r != y {
                mismatched_top_inputs.push(x);
            }

            xy_and_nodes.push(x_and.o);
            xy_xor_nodes.push(x_xor.o);
        });

    debug_assert!(mismatched_top_inputs
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec()
        .is_empty());

    debug_assert_eq!(xy_xor_nodes[0], z_nodes[0]);

    debug_assert!(
        edges_by_output[z_nodes[1]].unwrap().l == xy_xor_nodes[1]
            || edges_by_output[z_nodes[1]].unwrap().r == xy_xor_nodes[1]
    );

    debug_assert!(
        edges_by_output[z_nodes[1]].unwrap().l == xy_and_nodes[0]
            || edges_by_output[z_nodes[1]].unwrap().r == xy_and_nodes[0]
    );

    let mut carry_nodes_2 = vec![None; z_nodes.len()];

    xy_and_nodes.iter().copied().enumerate().for_each(|(i, n)| {
        let c = edges_by_inputs[n]
            .iter()
            .find_map(|e| (e.g == G::O).then_some(e.o));
        carry_nodes_2[i] = c;
    });

    let mut carry_nodes = vec![None; z_nodes.len()];

    let wrong_top_bottom = z_nodes
        .iter()
        .copied()
        .enumerate()
        .skip(1)
        .filter(|(_, z)| !wrong_output_gate.contains(z))
        .filter(|(i, _)| *i != x_nodes.len())
        .filter_map(|(i, z)| {
            let edge = edges_by_output[z].unwrap();

            if edge.l == xy_xor_nodes[i] {
                carry_nodes[i - 1] = Some(edge.r);
                None
            } else if edge.r == xy_xor_nodes[i] {
                carry_nodes[i - 1] = Some(edge.l);
                None
            } else {
                // match (
                //     edges_by_output[edge.l].unwrap().g,
                //     edges_by_output[edge.r].unwrap().g,
                // ) {
                //     (G::A, G::A) => {}
                //     (G::A, _) => carry_nodes[i - 1] = Some(edge.l),
                //     (_, G::A) => carry_nodes[i - 1] = Some(edge.r),
                //     _ => {}
                // }
                Some(z)
            }
        })
        .collect_vec();

    carry_nodes_2
        .iter()
        .copied()
        .enumerate()
        .for_each(|(i, n)| {
            if carry_nodes[i].is_none() {
                carry_nodes[i] = n;
            }
        });

    dbg!(carry_nodes
        .iter()
        .copied()
        .map(|n| n.map(|v| names[v]))
        .zip(carry_nodes_2.iter().copied().map(|n| n.map(|v| names[v])))
        .filter(|(l, r)| l != r)
        .collect_vec());

    dbg!(wrong_top_bottom
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec());

    let mut inter_nodes_2 = vec![None; z_nodes.len()];

    xy_xor_nodes.iter().copied().enumerate().for_each(|(i, n)| {
        let c = edges_by_inputs[n]
            .iter()
            .find_map(|e| (e.g == G::A).then_some(e.o));
        inter_nodes_2[i] = c;
    });

    let mut inter_nodes = vec![None; z_nodes.len()];

    let wrong_carry_nodes = carry_nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.map(|v| (i, v)))
        .filter_map(|(i, n)| {
            let edge = edges_by_output[n].unwrap();

            if n == xy_and_nodes[0] {
                return None;
            }

            if edge.g != G::O {
                has_needs
                    .entry((edge.g, G::O))
                    .and_modify(|v| v.push(n))
                    .or_insert(vec![n]);
                Some(n)
            } else {
                if edge.l == xy_and_nodes[i] {
                    inter_nodes[i] = Some(edge.r);
                } else if edge.r == xy_and_nodes[i] {
                    inter_nodes[i] = Some(edge.l);
                } else {
                    match (
                        edges_by_output[edge.l].unwrap().g,
                        edges_by_output[edge.r].unwrap().g,
                    ) {
                        (G::A, G::A) => {}
                        (G::A, _) => inter_nodes[i - 1] = Some(edge.l),
                        (_, G::A) => inter_nodes[i - 1] = Some(edge.r),
                        _ => {}
                    }
                }
                None
            }
        })
        .collect_vec();

    inter_nodes_2
        .iter()
        .copied()
        .enumerate()
        .for_each(|(i, n)| {
            if inter_nodes[i].is_none() {
                inter_nodes[i] = n;
            }
        });

    dbg!(inter_nodes
        .iter()
        .copied()
        .map(|n| n.map(|v| names[v]))
        .zip(inter_nodes_2.iter().copied().map(|n| n.map(|v| names[v])))
        .filter(|(l, r)| l != r)
        .collect_vec());

    dbg!(wrong_carry_nodes
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec());

    let mut inter_2_nodes = vec![None; z_nodes.len()];

    let wrong_inter_nodes = inter_nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.map(|v| (i, v)))
        .filter_map(|(i, n)| {
            let edge = edges_by_output[n].unwrap();
            if edge.g != G::A {
                has_needs
                    .entry((edge.g, G::A))
                    .and_modify(|v| v.push(n))
                    .or_insert(vec![n]);
                Some(n)
            } else {
                if edge.l == xy_xor_nodes[i] {
                    inter_2_nodes[i] = Some(edge.r);
                } else if edge.r == xy_xor_nodes[i] {
                    inter_2_nodes[i] = Some(edge.l);
                } else {
                    match (
                        edges_by_output[edge.l].unwrap().g,
                        edges_by_output[edge.r].unwrap().g,
                    ) {
                        (G::X, G::X) => {
                            unreachable!()
                        }
                        (G::X, _) => inter_2_nodes[i - 1] = Some(edge.l),
                        (_, G::X) => inter_2_nodes[i - 1] = Some(edge.r),
                        _ => return Some(n),
                    }
                }
                None
            }
        })
        .collect_vec();

    dbg!(wrong_inter_nodes
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec());

    let mut last_nodes = vec![None; z_nodes.len()];

    let wrong_inter_2_nodes = inter_2_nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.map(|v| (i, v)))
        .filter_map(|(i, n)| {
            let edge = edges_by_output[n].unwrap();

            if edge.g != G::O || Some(edge.o) != carry_nodes[i - 1] {
                has_needs
                    .entry((edge.g, G::O))
                    .and_modify(|v| v.push(n))
                    .or_insert(vec![n]);
                Some(n)
            } else {
                if edge.l == xy_and_nodes[i - 1] {
                    last_nodes[i] = Some(edge.r);
                } else if edge.r == xy_and_nodes[i - 1] {
                    last_nodes[i] = Some(edge.l);
                }
                None
            }
        })
        .collect_vec();

    dbg!(wrong_inter_2_nodes
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec());

    let wrong_last_nodes = last_nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.map(|v| (i, v)))
        .filter_map(|(i, n)| {
            let edge = edges_by_output[n].unwrap();
            if edge.g != G::A {
                has_needs
                    .entry((edge.g, G::A))
                    .and_modify(|v| v.push(n))
                    .or_insert(vec![n]);
                Some(n)
            } else {
                // if edge.l == xy_and_nodes[i - 2] {
                //     last_nodes[i] = Some(edge.r);
                // } else if edge.r == xy_and_nodes[i - 2] {
                //     last_nodes[i] = Some(edge.l);
                // }
                None
            }
        })
        .collect_vec();

    dbg!(wrong_last_nodes
        .iter()
        .copied()
        .map(|i| names[i])
        .collect_vec());

    let maybe_edges = wrong_output_gate
        .iter()
        .copied()
        .chain(wrong_top_bottom.iter().copied())
        .chain(wrong_carry_nodes.iter().copied())
        .chain(wrong_inter_nodes.iter().copied())
        .chain(wrong_inter_2_nodes.iter().copied())
        .chain(wrong_last_nodes.iter().copied())
        .flat_map(|n| {
            edges_by_output[n]
                .iter()
                .copied()
                .copied()
                .chain(edges_by_inputs[n].iter().copied())
        })
        .collect::<HashSet<_>>();

    let multiple_appearances = maybe_edges
        .iter()
        .flat_map(|e| e.nodes())
        .map(|n| (n, ()))
        .into_group_map()
        .into_iter()
        .map(|(n, v)| (n, v.len()))
        .sorted_by(|l, r| r.1.cmp(&l.1))
        //.take(10)
        .collect_vec();

    //dbg!(&multiple_appearances);
    let result = wrong_output_gate
        .into_iter()
        .chain(wrong_top_bottom)
        .chain(wrong_carry_nodes)
        .chain(wrong_inter_nodes)
        .chain(wrong_inter_2_nodes)
        .chain(wrong_last_nodes)
        .map(|n| names[n])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .join(",");

    dbg!(has_needs
        .iter()
        .map(|(k, v)| (k, v.iter().map(|n| names[*n]).collect_vec()))
        .collect_vec());

    dbg!(result);

    // dbg!(&carry_nodes);
    // dbg!(&inter_nodes);
    //dbg!(maybe_edges);

    None
}

// gct,kvf,nvh,tdb,z12,z19,z23,z37
// gct,nvh,vvf,z12,z19,z23,z37
// fgn,gct,nvh,vvf,z12,z19,z23,z37
// fgn,gct,nvh,pdw,vvf,z12,z19,z23,z37
// fgn,nvh,pdw,vvf,z12,z19,z23,z37

// vvf, z19, fwj, kjm, fvt, hcm, z37, nvh, fgn, shg

// z37, hcm, z19, kjm, z12, z23, nvh,

// x3------x---a-x2--a---x--x1---a-----x------y1
//         |   |     |   |       |     |
//         |   o---- m2  |       |     |
//         |   |         |       |     |
//         x --c2        x-------c1    |
//         |             |             |
//         z3            z2            z1
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
