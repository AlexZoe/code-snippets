use colored::Colorize;
use ndarray::{array, Array, Array2};
use std::collections::VecDeque;

type Distance = u64;
type Rows = usize;
type Columns = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Location {
    coord: Coord,
    elevation: i8,
    visited: bool,
    from: Option<Coord>,
}

impl Location {
    fn new(elevation: i8, coord: Coord) -> Self {
        Self {
            coord,
            elevation,
            visited: false,
            from: None,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = (self.elevation + 96) as u8 as char;
        write!(
            f,
            "{}",
            if self.visited {
                c.to_string().blue()
            } else {
                c.to_string().red()
            }
        )
    }
}

pub struct Map {
    map: Array2<Location>,
    start: Coord,
    goal: Coord,
}

impl Map {
    pub fn new(input: &str) -> Self {
        create_map_from_str(input)
    }

    fn get_visited_neighbors(&mut self, coord: &Coord) -> Vec<Coord> {
        let center = &self.map[[coord.y, coord.x]].clone();

        let (dimy, dimx) = self.map.dim();

        let mut neighbors = Vec::new();

        for x in (if coord.x == 0 { 0 } else { coord.x - 1 })..(if coord.x == dimx - 1 {
            coord.x + 1
        } else {
            coord.x + 2
        }) {
            if x != coord.x {
                let ref mut candidate = &mut self.map[[coord.y, x]];
                let diff = candidate.elevation - center.elevation;
                if !candidate.visited && (diff >= -1) {
                    candidate.visited = true;
                    candidate.from = Some(coord.clone());
                    neighbors.push(Coord { y: coord.y, x });
                }
            }
        }

        for y in (if coord.y == 0 { 0 } else { coord.y - 1 })..(if coord.y == dimy - 1 {
            coord.y + 1
        } else {
            coord.y + 2
        }) {
            if y != coord.y {
                let ref mut candidate = &mut self.map[[y, coord.x]];
                let diff = candidate.elevation - center.elevation;
                if !candidate.visited && (diff >= -1) {
                    candidate.visited = true;
                    candidate.from = Some(coord.clone());
                    neighbors.push(Coord { y, x: coord.x });
                }
            }
        }
        neighbors
    }

    fn calculate_distance(&self, start: &Coord, goal: &Coord) -> Option<Distance> {
        let mut dist = 0;
        let mut next = self.map[[goal.y, goal.x]].from;
        while let Some(prev) = next {
            dist += 1;
            if prev == *start {
                return Some(dist);
            }
            next = self.map[[prev.y, prev.x]].from
        }
        None
    }

    pub fn search(&mut self, print: bool) -> Option<Distance> {
        let mut eval: VecDeque<Coord> = VecDeque::new();
        self.map[[self.goal.y, self.goal.x]].visited = true;
        eval.push_back(self.goal.clone());

        while eval.len() != 0 {
            if print {
                self.print();
            }
            let next = &eval.pop_front().unwrap();
            if next == &self.start {
                return self.calculate_distance(&self.goal, &self.start);
            }
            eval.extend(self.get_visited_neighbors(next));
        }
        None
    }

    pub fn inverse_search_to_elevation(&mut self, elevation: i8, print: bool) -> Option<Distance> {
        let mut eval: VecDeque<Coord> = VecDeque::new();
        self.map[[self.goal.y, self.goal.x]].visited = true;
        eval.push_back(self.goal.clone());

        while eval.len() != 0 {
            if print {
                self.print();
            }
            let next = &eval.pop_front().unwrap();
            if self.map[[next.y, next.x]].elevation == elevation {
                return self.calculate_distance(&self.goal, &next);
            }
            eval.extend(self.get_visited_neighbors(next));
        }
        None
    }

    pub fn set_start(&mut self, start: Coord) {
        self.start = start;
    }

    pub fn set_goal(&mut self, goal: Coord) {
        self.goal = goal;
    }

    fn print(&self) {
        let shape = self.map.raw_dim();
        for y in 0..shape[0] {
            for x in 0..shape[1] {
                print!("{}", self.map[[y, x]]);
            }
            println!("");
        }
        println!("\n");
    }
}

fn parse_map_dimensions(input: &str) -> (Rows, Columns) {
    let rows = input.lines().count();
    let columns = input
        .chars()
        .take_while(|c: &char| c.is_alphabetic())
        .count();
    (rows, columns)
}

fn create_map_from_str(input: &str) -> Map {
    let mut start = Coord { x: 0, y: 0 };
    let mut goal = Coord { x: 0, y: 0 };
    let (rows, columns) = parse_map_dimensions(input);
    let chars: Vec<i8> = input
        .chars()
        .filter(|c: &char| c.is_alphabetic())
        .enumerate()
        .map(|(i, c)| {
            if c == 'S' {
                start = Coord {
                    x: i % columns,
                    y: i / columns,
                };
                1 as i8
            } else if c == 'E' {
                goal = Coord {
                    x: i % columns,
                    y: i / columns,
                };
                26 as i8
            } else {
                c as i8 - 96
            }
        })
        .collect();
    Map {
        map: Array::from_shape_fn((rows, columns), |(y, x)| {
            Location::new(chars[y * columns + x], Coord { x, y })
        }),
        start,
        goal,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn get_dimensions_of_map() {
        let input = "ad\nbc";
        assert_eq!((2, 2), parse_map_dimensions(input));
    }

    #[test]
    fn map_from_string_matches_expects() {
        let input = "SE\nbc";
        let expected = array![
            [
                Location::new(1, Coord { x: 0, y: 0 }),
                Location::new(26, Coord { x: 1, y: 0 })
            ],
            [
                Location::new(2, Coord { x: 0, y: 1 }),
                Location::new(3, Coord { x: 1, y: 1 })
            ]
        ];
        assert_eq!(expected, create_map_from_str(&input).map);
    }

    #[test]
    fn get_neighbors_within_elevation_range() {
        let input = "ad\nbc";
        let mut map = Map::new(&input);
        assert_eq!(
            vec!(Coord { x: 1, y: 1 },),
            map.get_visited_neighbors(&Coord { x: 1, y: 0 })
        );
    }

    #[test]
    fn get_distance_to_start() {
        let input = "ad\nbc";
        let mut map = Map::new(&input);
        map.set_start(Coord { x: 0, y: 0 });
        map.set_goal(Coord { x: 1, y: 0 });
        assert_eq!(3, map.search(false).unwrap());
    }

    #[test]
    fn distance_from_sample_input() {
        let input = fs::read_to_string("./assets/day12_sample.txt").expect("Read ./test_input");
        let mut map = Map::new(&input);
        assert_eq!(31, map.search(false).unwrap());
    }

    #[test]
    fn distance_from_sample_input_to_elevation() {
        let input = fs::read_to_string("./assets/day12_sample.txt").expect("Read ./test_input");
        let mut map = Map::new(&input);
        assert_eq!(29, map.inverse_search_to_elevation(1, false).unwrap());
    }
}
