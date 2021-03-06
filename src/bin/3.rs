use advent_of_code_2019::coordinates::two_d::{Point, PointLike, ZERO_POINT};
use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use env_logger::Env;
use std::ops::RangeInclusive;

struct Three {}

#[derive(Debug, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn new(c: char) -> Option<Dir> {
        match c {
            'U' => Some(Dir::Up),
            'R' => Some(Dir::Right),
            'D' => Some(Dir::Down),
            'L' => Some(Dir::Left),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Wire {
    None,
    Center,
    One,
    Two,
    Intersection,
}

impl Default for Wire {
    fn default() -> Self {
        Wire::None
    }
}

#[derive(Debug, Clone)]
struct Move {
    dir: Dir,
    distance: usize,
}

impl Move {
    fn new(s: &str) -> Option<Move> {
        let maybe_dir = s.chars().next().and_then(Dir::new);

        maybe_dir.map(|dir| Move {
            dir,
            distance: s[1..].parse::<usize>().expect("parse error"),
        })
    }
}

impl Problem for Three {
    type Input = Vec<Vec<Move>>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|w| {
                w.split(',')
                    .map(|s| Move::new(s).expect("parse error"))
                    .collect()
            })
            .collect::<Vec<Vec<Move>>>()
    }

    fn part_1(wires: &Vec<Vec<Move>>, state: &ProblemState<Self::Extra>) -> Option<String> {
        let (width, height) = calculate_max_dimensions(wires);
        let mut grid = Grid::new_from_inclusive_range(width, height);
        grid.set(0, 0, Wire::Center);

        let mut point = Point { x: 0, y: 0 };
        for Move { dir, distance } in &wires[0] {
            for _ in 0..*distance {
                match dir {
                    Dir::Up => point.y += 1,
                    Dir::Right => point.x += 1,
                    Dir::Down => point.y -= 1,
                    Dir::Left => point.x -= 1,
                };

                grid.set(point.x, point.y, Wire::One);
            }
        }

        let mut min_distance = std::usize::MAX;
        point = Point { x: 0, y: 0 };
        for Move { dir, distance } in &wires[1] {
            for _ in 0..*distance {
                match dir {
                    Dir::Up => point.y += 1,
                    Dir::Right => point.x += 1,
                    Dir::Down => point.y -= 1,
                    Dir::Left => point.x -= 1,
                };

                if *grid.get(point.x, point.y) == Wire::One {
                    let distance = point.distance(&ZERO_POINT);
                    if min_distance > distance {
                        min_distance = distance;
                    }

                    grid.set(point.x, point.y, Wire::Intersection);
                } else {
                    grid.set(point.x, point.y, Wire::Two);
                }
            }
        }

        if log::log_enabled!(log::Level::Debug) {
            grid.write_image(&*format!("./{}.png", state.name), |w| match w {
                Wire::Center => [255, 255, 255, 255],
                Wire::One => [255, 0, 0, 255],
                Wire::Two => [0, 0, 255, 255],
                Wire::Intersection => [0, 255, 0, 255],
                Wire::None => [0, 0, 0, 255],
            });
        }

        Some(format!("{:?}", min_distance))
    }

    fn part_2(wires: &Vec<Vec<Move>>, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let (width, height) = calculate_max_dimensions(wires);
        let mut grid = Grid::new_from_inclusive_range(width, height);
        grid.set(0, 0, (Wire::Center, 0));

        let mut point = Point { x: 0, y: 0 };
        let mut steps = 1;
        for Move { dir, distance } in &wires[0] {
            for _ in 0..*distance {
                match dir {
                    Dir::Up => point.y += 1,
                    Dir::Right => point.x += 1,
                    Dir::Down => point.y -= 1,
                    Dir::Left => point.x -= 1,
                };

                grid.set(point.x, point.y, (Wire::One, steps));
                steps += 1;
            }
        }

        let mut min_distance = std::usize::MAX;
        point = Point { x: 0, y: 0 };
        steps = 1;
        for Move { dir, distance } in &wires[1] {
            for _ in 0..*distance {
                match dir {
                    Dir::Up => point.y += 1,
                    Dir::Right => point.x += 1,
                    Dir::Down => point.y -= 1,
                    Dir::Left => point.x -= 1,
                };

                if let (Wire::One, other_segment) = *grid.get(point.x, point.y) {
                    let distance = other_segment + steps;
                    if min_distance > distance {
                        min_distance = distance;
                    }

                    grid.set(point.x, point.y, (Wire::Intersection, steps));
                } else {
                    grid.set(point.x, point.y, (Wire::Two, steps));
                }
                steps += 1;
            }
        }

        Some(format!("{:?}", min_distance))
    }

    fn problem_number() -> usize {
        3
    }
}

fn calculate_max_dimensions(wires: &[Vec<Move>]) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    for wire in wires {
        let mut x: isize = 0;
        let mut y: isize = 0;
        for Move { dir, distance } in wire {
            match dir {
                Dir::Up => y += *distance as isize,
                Dir::Right => x += *distance as isize,
                Dir::Down => y -= *distance as isize,
                Dir::Left => x -= *distance as isize,
            };

            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
    }

    (x_min..=x_max, y_min..=y_max)
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Three;
        RunFor::Part1, (), "R8,U5,L5,D3\nU7,R6,D4,L4",
        RunFor::Both,(), "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
        RunFor::Both,(), "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
    );
    run::<Three>((), include_str!("3_input.txt"));
}

#[cfg(test)]
mod three {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    #[ignore] // this code is annoyingly slow
    fn test() {
        assert_solution::<Three>(include_str!("3_input.txt"), (), "4981", "164012");
    }
}
