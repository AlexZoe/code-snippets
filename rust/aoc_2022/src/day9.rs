use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug)]
pub enum Directions {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct WalkInstruction {
    direction: Directions,
    distance: u64,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn walk_one(&mut self, dir: &Directions) {
        match dir {
            Directions::Left => self.x -= 1,
            Directions::Right => self.x += 1,
            Directions::Up => self.y += 1,
            Directions::Down => self.y -= 1,
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Sub for &Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for &Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Grid {
    upper_left: Position,
    lower_right: Position,
}

pub struct RopeBridge {
    head: Position,
    tail: Vec<Position>,
    visited: HashSet<Position>,
    grid: Grid,
}

impl RopeBridge {
    pub fn new(tail_len: usize) -> Self {
        Self {
            head: Position { x: 0, y: 0 },
            tail: vec![Position { x: 0, y: 0 }; tail_len],
            visited: HashSet::from([Position { x: 0, y: 0 }]),
            grid: Grid {
                upper_left: Position { x: 0, y: 0 },
                lower_right: Position { x: 0, y: 0 },
            },
        }
    }

    pub fn visited_places(&self) -> usize {
        self.visited.len()
    }

    pub fn walk(&mut self, inst: WalkInstruction) {
        for _ in 0..inst.distance {
            self.head.walk_one(&inst.direction);
            self.update_tail();
            self.visited.insert(*self.tail.last().unwrap());
        }
    }

    #[allow(dead_code)]
    pub fn walk_with_print(&mut self, inst: WalkInstruction) {
        for _ in 0..inst.distance {
            self.head.walk_one(&inst.direction);
            self.update_grid();
            self.update_tail();
            println!("{}", self);
            self.visited.insert(*self.tail.last().unwrap());
        }
    }

    fn update_tail(&mut self) {
        for i in 0..self.tail.len() {
            let head = if i == 0 { self.head } else { self.tail[i - 1] };

            let mut diff = head - self.tail[i];

            if diff.x.abs() <= 1 && diff.y.abs() <= 1 {
                diff.x = 0;
                diff.y = 0;
            }

            if diff.x > 1 {
                diff.x = 1;
            } else if diff.x < -1 {
                diff.x = -1;
            } else {
            }

            if diff.y > 1 {
                diff.y = 1;
            } else if diff.y < -1 {
                diff.y = -1;
            } else {
            }
            self.tail[i] += diff;
        }
    }

    #[allow(dead_code)]
    fn update_grid(&mut self) {
        if self.head.x < self.grid.upper_left.x {
            self.grid.upper_left.x = self.head.x;
        }
        if self.head.y > self.grid.upper_left.y {
            self.grid.upper_left.y = self.head.y;
        }

        if self.head.x > self.grid.lower_right.x {
            self.grid.lower_right.x = self.head.x;
        }
        if self.head.y < self.grid.lower_right.y {
            self.grid.lower_right.y = self.head.y;
        }
    }
}

pub fn parse_walk_instructions(input: &str) -> Vec<WalkInstruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let items: Vec<&str> = line.split(' ').collect();
        let direction = match items[0] {
            "R" => Directions::Right,
            "L" => Directions::Left,
            "U" => Directions::Up,
            "D" => Directions::Down,
            _ => unreachable!(),
        };
        instructions.push(WalkInstruction {
            direction,
            distance: (items[1].parse::<u64>().unwrap_or(0)),
        })
    }
    instructions
}

impl Display for RopeBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.grid.lower_right.y + self.grid.upper_left.y.abs() + 1).rev() {
            for x in 0..self.grid.lower_right.x + self.grid.upper_left.x.abs() + 1 {
                let cur = Position {
                    x: x - self.grid.upper_left.x.abs(),
                    y: y - self.grid.lower_right.y.abs(),
                };
                if cur == self.head {
                    write!(f, "H")?;
                } else if self.tail.contains(&cur) {
                    for i in 0..self.tail.len() {
                        if cur == self.tail[i] {
                            write!(f, "{}", i + 1)?;
                            break;
                        }
                    }
                } else if cur.x == 0 && cur.y == 0 {
                    write!(f, "s")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod test {
    use super::super::file;
    use super::*;

    #[test]
    fn visited_places_of_tail_at_start_is_one() {
        let rope_bridge = RopeBridge::new(1);
        assert_eq!(1, rope_bridge.visited_places());
    }

    #[test]
    fn visited_places_two_after_moving_two_positions() {
        let mut rope_bridge = RopeBridge::new(1);
        rope_bridge.walk(WalkInstruction {
            direction: Directions::Right,
            distance: 2,
        });
        assert_eq!(2, rope_bridge.visited_places());
    }

    #[test]
    fn no_update_for_moving_single_step() {
        let mut rope_bridge = RopeBridge::new(1);
        rope_bridge.walk(WalkInstruction {
            direction: Directions::Right,
            distance: 1,
        });
        assert_eq!(1, rope_bridge.visited_places());
    }

    #[test]
    fn expect_test_sample_input() {
        let instructions =
            parse_walk_instructions(file::get_string_from_file("assets/day9_sample.txt").as_str());
        let mut rope_bridge = RopeBridge::new(1);

        for inst in instructions {
            rope_bridge.walk(inst);
        }
        assert_eq!(13, rope_bridge.visited_places());
    }

    #[test]
    fn expect_test_sample_input_long() {
        let instructions = parse_walk_instructions(
            file::get_string_from_file("assets/day9_sample_long.txt").as_str(),
        );
        let mut rope_bridge = RopeBridge::new(9);

        for inst in instructions {
            rope_bridge.walk(inst);
        }
        assert_eq!(36, rope_bridge.visited_places());
    }
}
