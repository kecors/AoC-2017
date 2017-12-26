use std::io;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    RIGHT,
    UP,
    LEFT,
    DOWN
}

#[derive(Debug)]
struct Square {
    x:         i32,
    y:         i32,
    direction: Direction,
    sum:       Option<u32>
}

impl Square {
    fn initial() -> Square {
        Square { x: 0, y: 0, direction: Direction::RIGHT, sum: None }
    }

    fn create_next(&self) -> Square {
        let (x, y) : (i32, i32) = match self.direction {
            Direction::RIGHT => {
                (self.x + 1, self.y    )
            },
            Direction::UP => {
                (self.x    , self.y + 1)
            },
            Direction::LEFT => {
                (self.x - 1, self.y    )
            },
            Direction::DOWN => {
                (self.x    , self.y - 1)
            },
        };

        Square { x: x, y: y, direction: self.direction, sum: None }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::RIGHT => Direction::UP,
            Direction::UP    => Direction::LEFT,
            Direction::LEFT  => Direction::DOWN,
            Direction::DOWN  => Direction::RIGHT
        }
    }

    fn calculate_steps(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

fn build_grid(grid: &mut Vec<Square>, limit: usize) {
    let mut square = Square::initial();
    let mut distance = 1;
    let mut step = 0;

    loop {
        let next_square = square.create_next();

        grid.push(square);

        square = next_square;

        // Calcuate sum here

        step += 1;
        if step == distance {
            step = 0;
            if square.direction == Direction::UP || square.direction == Direction::DOWN {
                distance += 1;
            }
            square.turn();
        }

        if grid.len() == limit { break; }
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let limit: usize = input.trim().parse::<usize>().unwrap();

    let mut grid: Vec<Square> = Vec::with_capacity(limit);

    build_grid(&mut grid, limit);

    println!("steps for {} = {}", limit, grid[limit-1].calculate_steps());
}
