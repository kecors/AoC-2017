use std::io::{stdin, Read};
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Condition {
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    unclean: HashMap<Position, Condition>,
    position: Position,
    direction: Direction,
    infection: u32,
}

impl State {
    fn new(unclean: HashMap<Position, Condition>) -> State {
        State {
            unclean: unclean,
            position: Position { x: 0, y: 0 },
            direction: Direction::Up,
            infection: 0,
        }
    }

    fn burst_part_1(&mut self) {
        let mut remove_flag = false;
        let mut insert_infected_flag = false;
        if let Some(_condition) = self.unclean.get(&self.position) {
            self.direction = match self.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            remove_flag = true;
        } else {
            self.direction = match self.direction {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            };
            insert_infected_flag = true;
            self.infection += 1;
        }
        if remove_flag {
            self.unclean.remove(&self.position);
        }
        if insert_infected_flag {
            self.unclean
                .insert(self.position.clone(), Condition::Infected);
        }
        match self.direction {
            Direction::Up => {
                self.position.y += 1;
            }
            Direction::Right => {
                self.position.x += 1;
            }
            Direction::Down => {
                self.position.y -= 1;
            }
            Direction::Left => {
                self.position.x -= 1;
            }
        }
    }

    fn burst_part_2(&mut self) {
        let mut remove_flag = false;
        let mut insert_weakened_flag = false;
        if let Some(condition) = self.unclean.get_mut(&self.position) {
            match *condition {
                Condition::Weakened => {
                    *condition = Condition::Infected;
                    self.infection += 1;
                }
                Condition::Infected => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                    *condition = Condition::Flagged;
                }
                Condition::Flagged => {
                    self.direction = match self.direction {
                        Direction::Up => Direction::Down,
                        Direction::Right => Direction::Left,
                        Direction::Down => Direction::Up,
                        Direction::Left => Direction::Right,
                    };
                    remove_flag = true;
                }
            }
        } else {
            self.direction = match self.direction {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            };
            insert_weakened_flag = true;
        }
        if remove_flag {
            self.unclean.remove(&self.position);
        }
        if insert_weakened_flag {
            self.unclean
                .insert(self.position.clone(), Condition::Weakened);
        }
        match self.direction {
            Direction::Up => {
                self.position.y += 1;
            }
            Direction::Right => {
                self.position.x += 1;
            }
            Direction::Down => {
                self.position.y -= 1;
            }
            Direction::Left => {
                self.position.x -= 1;
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let mut hm: HashMap<Position, Condition> = HashMap::new();
    let width = input.lines().count() as i32;
    let mut x: i32;
    let mut y: i32 = width / 2;
    for line in input.lines() {
        x = -1 * (width / 2);
        for c in line.chars() {
            if c == '#' {
                hm.insert(Position { x, y }, Condition::Infected);
            }
            x += 1;
        }
        y -= 1;
    }

    let mut state = State::new(hm.clone());
    for _ in 0..10000 {
        state.burst_part_1();
    }
    println!("part 1: infections = {}", state.infection);

    let mut state = State::new(hm);
    for _ in 0..10000000 {
        state.burst_part_2();
    }
    println!("part 2: infections = {}", state.infection);
}
