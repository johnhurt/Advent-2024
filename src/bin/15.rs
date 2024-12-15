use std::collections::HashMap;

use advent_of_code::{Compass, Grid};
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Obj {
    Box,
    BoxL,
    BoxR,
    Robot,
    Wall,
    None,
}

impl From<char> for Obj {
    fn from(value: char) -> Self {
        match value {
            '@' => Obj::Robot,
            '.' => Obj::None,
            'O' => Obj::Box,
            '#' => Obj::Wall,
            _ => unreachable!(),
        }
    }
}

impl From<Obj> for char {
    fn from(value: Obj) -> Self {
        match value {
            Obj::Box => 'O',
            Obj::BoxL => '[',
            Obj::BoxR => ']',
            Obj::None => '.',
            Obj::Robot => '@',
            Obj::Wall => '#',
        }
    }
}

fn parse_instr(i: u8) -> Option<Compass> {
    match i {
        b'>' => Some(Compass::E),
        b'<' => Some(Compass::W),
        b'^' => Some(Compass::N),
        b'v' => Some(Compass::S),
        _ => None,
    }
}

fn p1_step(robot: usize, dir: Compass, grid: &mut Grid<Obj>) -> usize {
    let mut first_box_opt = None;
    let mut first_gap = None;

    for (next_i, next) in grid.ray(robot, dir.into()).skip(1) {
        match next {
            Obj::Box => first_box_opt = first_box_opt.or(Some(next_i)),
            Obj::Wall => return robot,
            Obj::None => {
                first_gap = Some(next_i);
                break;
            }
            _ => unreachable!(),
        }
    }

    let first_gap = first_gap.unwrap();
    if let Some(first_box) = first_box_opt {
        grid.data[robot] = Obj::None;
        grid.data[first_box] = Obj::Robot;
        grid.data[first_gap] = Obj::Box;
        first_box
    } else {
        grid.data[robot] = Obj::None;
        grid.data[first_gap] = Obj::Robot;
        first_gap
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, inst) = input.split_once("\n\n").unwrap();

    let mut grid: Grid<Obj> = Grid::parse_lines(map);

    let robot = grid
        .data
        .iter()
        .enumerate()
        .find_map(|(i, o)| (*o == Obj::Robot).then_some(i))
        .unwrap();

    inst.bytes()
        .filter_map(parse_instr)
        // .fold(robot, |robot, dir| {
        //     let r = p1_step(robot, dir, &mut grid);
        //     println!("\n{dir}\n");
        //     grid.print();
        //     r
        // });
        .fold(robot, |robot, dir| p1_step(robot, dir, &mut grid));

    let result = grid
        .data
        .iter()
        .enumerate()
        .filter(|&(_, o)| (*o == Obj::Box))
        .map(|(i, _)| {
            let (c, r) = grid.to_col_row(i);
            100 * r + c
        })
        .sum();

    Some(result)
}

impl Obj {
    fn expand(self) -> [Obj; 2] {
        match self {
            Obj::Box => [Obj::BoxL, Obj::BoxR],
            Obj::Wall => [Obj::Wall, Obj::Wall],
            Obj::None => [Obj::None, Obj::None],
            Obj::Robot => [Obj::Robot, Obj::None],
            _ => unreachable!(),
        }
    }
}

fn p2_step(robot: usize, dir: Compass, grid: &mut Grid<Obj>) -> usize {
    use Compass as D;
    let mut try_list = vec![robot];
    let mut resolution = HashMap::new();

    while let Some(curr) = try_list.pop() {
        let next = grid.step_from_index(curr, dir).unwrap();
        resolution.insert(next, grid.data[curr]);
        match (dir, grid.data[next]) {
            (_, Obj::Wall) => return robot,
            (_, Obj::None) => {}
            (D::E | D::W, Obj::BoxL | Obj::BoxR) => {
                try_list.push(next);
            }
            (D::N | D::S, Obj::BoxL) => {
                try_list.push(next);
                try_list.push(next + 1);
                resolution.entry(next + 1).or_insert(Obj::None);
            }
            (D::N | D::S, Obj::BoxR) => {
                try_list.push(next);
                try_list.push(next - 1);
                resolution.entry(next - 1).or_insert(Obj::None);
            }
            _ => unreachable!(),
        }
    }

    resolution.into_iter().for_each(|(i, v)| grid.data[i] = v);
    grid.step_from_index(robot, dir).unwrap()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, inst) = input.split_once("\n\n").unwrap();

    let grid: Grid<Obj> = Grid::parse_lines(map);
    let new_width = grid.width * 2;
    let mut grid = Grid::new(
        grid.data.into_iter().flat_map(Obj::expand).collect_vec(),
        new_width,
    );

    let robot = grid
        .data
        .iter()
        .enumerate()
        .find_map(|(i, o)| (*o == Obj::Robot).then_some(i))
        .unwrap();

    grid.data[robot] = Obj::None;

    inst.bytes()
        .filter_map(parse_instr)
        // .fold(robot, |robot, dir| {
        //     let r = p2_step(robot, dir, &mut grid);
        //     println!("\n{dir}\n");
        //     grid.print();
        //     r
        // });
        .fold(robot, |robot, dir| p2_step(robot, dir, &mut grid));

    let result = grid
        .data
        .iter()
        .enumerate()
        .filter(|&(_, o)| (*o == Obj::BoxL))
        .map(|(i, _)| {
            let (c, r) = grid.to_col_row(i);
            100 * r + c
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_small() {
        let result = part_one(
            &advent_of_code::template::read_extra_example_file(DAY, 2),
        );
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
