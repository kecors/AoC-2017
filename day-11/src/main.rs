use std::io;

#[derive(Debug)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW
}

#[derive(Debug, Default)]
struct State {
    x: i32,
    y: i32,
    z: i32,
    maximum_distance: i32
}

impl State {
    fn new() -> State {
        State::default()
    }

    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::N  => { self.x += 1; self.z -= 1; },
            Direction::NE => { self.x += 1; self.y -= 1; },
            Direction::SE => { self.z += 1; self.y -= 1; },
            Direction::S  => { self.z += 1; self.x -= 1; },
            Direction::SW => { self.y += 1; self.x -= 1; },
            Direction::NW => { self.y += 1; self.z -= 1; },
        }
        self.update_maximum_distance();
    }

    fn update_maximum_distance(&mut self) {
        for distance in [self.x.abs(), self.y.abs(), self.z.abs()].iter() {
            if *distance > self.maximum_distance {
                self.maximum_distance = *distance;
            }
        }
    }

    fn fewest_steps(&self) -> i32 {
        *[self.x.abs(), self.y.abs(), self.z.abs()].iter().max().unwrap()
    }
}

fn parse_line(input: &str) -> Vec<Direction> {
    let mut steps = Vec::new();

    let directions: Vec<&str> = input.split(',').collect();
    for direction in directions {
        match direction {
            "n"  => { steps.push(Direction::N); },
            "ne" => { steps.push(Direction::NE); },
            "se" => { steps.push(Direction::SE); },
            "s"  => { steps.push(Direction::S); },
            "sw" => { steps.push(Direction::SW); },
            "nw" => { steps.push(Direction::NW); },
            _    => { }
        }
    }

    steps
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
//    println!("input = {:?}", input);

    let directions = parse_line(input.trim());
//    println!("directions = {:?}", directions);

    let mut state = State::new();

    for direction in directions {
        state.go(direction);
    }
//    println!("state = {:?}", state);

    println!("fewest steps = {}", state.fewest_steps());
    println!("maximum distance = {}", state.maximum_distance);
}
