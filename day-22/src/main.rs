use std::io::{stdin, Read};
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct State {
    infected: HashSet<Position>,
    position: Position,
    direction: Direction,
    infection: u32
}

impl State {
    fn new(infected: HashSet<Position>) -> State {
        State {
            infected: infected,
            position: Position { x: 0, y: 0 },
            direction: Direction::UP,
            infection: 0
        }
    }

    fn burst(&mut self) {
        if self.infected.contains(&self.position) {
            self.direction = match self.direction {
                Direction::UP    => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN  => Direction::LEFT,
                Direction::LEFT  => Direction::UP
            };
            self.infected.remove(&self.position);
        } else {
            self.direction = match self.direction {
                Direction::UP    => Direction::LEFT,
                Direction::LEFT  => Direction::DOWN,
                Direction::DOWN  => Direction::RIGHT,
                Direction::RIGHT => Direction::UP
            };
            self.infected.insert(self.position.clone());
            self.infection += 1;
        }
        match self.direction {
            Direction::UP    => { self.position.y += 1; },
            Direction::RIGHT => { self.position.x += 1; },
            Direction::DOWN  => { self.position.y -= 1; },
            Direction::LEFT  => { self.position.x -= 1; }
        }
    }
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let mut hs: HashSet<Position> = HashSet::new();
    let width = input.lines().count() as i32;
    let mut x: i32;
    let mut y: i32 = width / 2;
    for line in input.lines() {
        x = -1 * (width / 2);
        for c in line.chars() {
            if c == '#' {
                hs.insert(Position { x, y });
            }
            x += 1;
        }
        y -= 1;
    }

    let mut state = State::new(hs);
//    println!("state = {:?}", state);

    for _ in 0..10000 {
        state.burst();
    }
//    println!("state = {:?}", state);
    println!("part 1: infections = {}", state.infection);
}
