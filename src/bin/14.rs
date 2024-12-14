advent_of_code::solution!(14);
use advent_of_code::Grid;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i64},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_line(line: &str) -> Robot {
    let result: IResult<_, _> = map(
        separated_pair(
            preceded(tag("p="), separated_pair(i64, char(','), i64)),
            tag(" v="),
            separated_pair(i64, char(','), i64),
        ),
        |(p, v)| Robot::from_parsed(p, v),
    )(line);

    result.expect("😍").1
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn from_parsed(p: (i64, i64), v: (i64, i64)) -> Self {
        Robot {
            px: p.0,
            py: p.1,
            vx: v.0,
            vy: v.1,
        }
    }

    fn after_time(self, t: i64, size: (i64, i64)) -> Self {
        let (sx, sy) = size;
        let Robot {
            mut px,
            mut py,
            vx,
            vy,
        } = self;

        px = (px + vx * t).rem_euclid(sx);
        py = (py + vy * t).rem_euclid(sy);

        Robot { px, py, vx, vy }
    }

    fn quadrant(self, size: (i64, i64)) -> Option<i64> {
        let (sx, sy) = size;
        let Robot { px, py, .. } = self;
        let bx = sx / 2;
        let by = sy / 2;

        let qx = (px != bx).then_some(px / (bx + 1))?;
        let qy = (py != by).then_some(py / (by + 1))?;

        Some(2 * qy + qx)
    }
}

fn p1_sized(input: &str, size: (i64, i64)) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(|r| r.after_time(100, size))
        .filter_map(|r| r.quadrant(size))
        .fold([0; 4], |mut acc, curr| {
            acc[curr as usize] += 1;
            acc
        })
        .into_iter()
        .product()
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = p1_sized(input, (101, 103));

    Some(result)
}

fn entropy(robots: &[Robot], size: (i64, i64)) -> f64 {
    let buckets_w = size.0 / 5 + 1;
    let bucket_count = buckets_w * (size.1 / 5 + 1);

    let buckets = robots.iter().map(|r| r.px / 5 + r.py / 5 * buckets_w).fold(
        vec![0.; bucket_count as usize],
        |mut acc, curr| {
            acc[curr as usize] += 1.;
            acc
        },
    );

    buckets.into_iter().map(|c: f64| (c - 1.).abs()).sum()
}

/// For the vibes
#[allow(dead_code)]
fn render(robots: &[Robot], size: (i64, i64)) {
    let mut grid =
        Grid::new(vec![' '; (size.0 * size.1) as usize], size.0 as usize);

    robots.iter().for_each(|r| {
        let i = grid.to_index(r.px as usize, r.py as usize);
        grid.data[i] = '#';
    });

    grid.print();
}

pub fn part_two(input: &str) -> Option<i64> {
    let size = (101, 103);
    let mut robots = input.lines().map(parse_line).collect_vec();
    // let mut originals = robots.clone();

    let (max_i, _) = (1..10_000).fold((0, 0.), |(max_i, max_ent), i| {
        robots.iter_mut().for_each(|r| *r = r.after_time(1, size));
        let ent = entropy(&robots, size);
        if ent > max_ent {
            (i, ent)
        } else {
            (max_i, max_ent)
        }
    });

    // originals
    //     .iter_mut()
    //     .for_each(|r| *r = r.after_time(max_i, size));
    // render(&originals, size);

    Some(max_i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = p1_sized(
            &advent_of_code::template::read_file("examples", DAY),
            (11, 7),
        );
        assert_eq!(result, 12);
    }
}
