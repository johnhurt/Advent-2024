use std::collections::{HashMap, HashSet};

use advent_of_code::ws;
use itertools::Itertools;
use nom::{
    character::complete::{anychar, char},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, tuple},
    IResult,
};

advent_of_code::solution!(23);

fn computer_to_u16(cmp: &str) -> IResult<&'_ str, u16> {
    map(tuple((anychar, anychar)), |(l, r)| {
        ((l as u16) << 8) + r as u16
    })(cmp)
}

fn parse_edges(input: &str) -> Vec<(u16, u16)> {
    let result: IResult<_, _> = many1(ws(separated_pair(
        computer_to_u16,
        char('-'),
        computer_to_u16,
    )))(input);
    result.expect("🎱").1
}

fn starts_with_t(c: u16) -> bool {
    c >> 8 == b't' as u16
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = HashMap::<u16, Vec<u16>>::new();
    let edges = parse_edges(input)
        .into_iter()
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .inspect(|(l, r)| {
            graph
                .entry(*l)
                .and_modify(|es| es.push(*r))
                .or_insert(vec![*r]);
        })
        .collect::<HashSet<_>>();

    let result = graph
        .into_iter()
        .filter(|(f, _)| starts_with_t(*f))
        .flat_map(|(f, ts)| {
            ts.iter()
                .copied()
                .cartesian_product(ts.iter().copied())
                .filter(|(l, r)| l != r)
                .filter(|(l, r)| edges.contains(&(*l, *r)))
                .map(|(l, r)| {
                    let mut triple = [f, l, r];
                    triple.sort();
                    triple
                })
                .collect_vec()
        })
        .collect::<HashSet<_>>()
        .len();

    Some(result)
}

fn find_incremented_clique(
    clique_list: &[u16],
    clique_set: &HashSet<u16>,
    edges: &HashSet<(u16, u16)>,
    nodes: &HashMap<u16, Vec<u16>>,
) -> HashMap<Vec<u16>, HashSet<u16>> {
    let clique_neighbors = clique_list
        .iter()
        .filter_map(|n| nodes.get(n))
        .flat_map(|ns| ns.iter().copied())
        .filter(|n| !clique_set.contains(n))
        .collect::<HashSet<_>>();

    clique_neighbors
        .into_iter()
        .filter(|l| clique_list.iter().all(|r| edges.contains(&(*l, *r))))
        .map(|n| {
            let mut new_clique_list = clique_list.to_vec();
            let mut new_clique_set = clique_set.clone();
            new_clique_list.push(n);
            new_clique_list.sort();
            new_clique_set.insert(n);
            (new_clique_list, new_clique_set)
        })
        .collect()
}

fn find_next_cliques(
    cliques: &HashMap<Vec<u16>, HashSet<u16>>,
    edges: &HashSet<(u16, u16)>,
    nodes: &HashMap<u16, Vec<u16>>,
) -> Option<HashMap<Vec<u16>, HashSet<u16>>> {
    let result = cliques
        .iter()
        .flat_map(|(clique_list, clique_set)| {
            find_incremented_clique(clique_list, clique_set, edges, nodes)
        })
        .collect::<HashMap<_, _>>();

    (!result.is_empty()).then_some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut graph = HashMap::<u16, Vec<u16>>::new();
    let edges = parse_edges(input)
        .into_iter()
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .inspect(|(l, r)| {
            graph
                .entry(*l)
                .and_modify(|es| es.push(*r))
                .or_insert(vec![*r]);
        })
        .collect::<HashSet<_>>();

    let mut cliques = edges
        .iter()
        .copied()
        .map(|(l, r)| {
            let mut clique_list = vec![l, r];
            let clique_set =
                clique_list.iter().copied().collect::<HashSet<_>>();
            clique_list.sort();
            (clique_list, clique_set)
        })
        .collect::<HashMap<_, _>>();

    while let Some(next_cliques) = find_next_cliques(&cliques, &edges, &graph) {
        cliques = next_cliques;
    }

    let result = cliques
        .into_keys()
        .next()
        .unwrap()
        .iter()
        .copied()
        .flat_map(|as_u16| {
            [
                ',',
                (as_u16 >> 8) as u8 as char,
                (as_u16 & 0xff) as u8 as char,
            ]
        })
        .skip(1)
        .join("");

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_owned()));
    }
}
