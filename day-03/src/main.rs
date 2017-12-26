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

#[derive(Clone)]
#[derive(Debug)]
struct Square {
    x:         i32,
    y:         i32,
    direction: Direction,
    sum:       u32
}

fn create_next_square(square: &Square) -> Square {
    let (x, y) : (i32, i32) = match square.direction {
        Direction::RIGHT => {
            (square.x + 1, square.y    )
        },
        Direction::UP => {
            (square.x    , square.y + 1)
        },
        Direction::LEFT => {
            (square.x - 1, square.y    )
        },
        Direction::DOWN => {
            (square.x    , square.y - 1)
        },
    };

    Square { x: x, y: y, direction: square.direction, sum: 0 }
}

fn build_grid(grid: &mut Vec<Square>, limit: usize) {
    let mut square = Square { x: 0, y: 0, direction: Direction::RIGHT, sum: 0 };
    let mut distance = 1;
    let mut index: usize = 0;
    let mut step = 0;

    loop {
        let next_square = create_next_square(&square);

        grid.push(square);

        square = next_square;

        // Calcuate sum here

        step += 1;
        if step == distance {
            // Reset step, possibly increment distance, and change direction
            step = 0;
            if square.direction == Direction::UP || square.direction == Direction::DOWN {
                distance += 1;
            }
            square.direction = match square.direction {
                Direction::RIGHT => Direction::UP,
                Direction::UP    => Direction::LEFT,
                Direction::LEFT  => Direction::DOWN,
                Direction::DOWN  => Direction::RIGHT
            }
        }

        index += 1;
        if index == limit { break; }
    }
}

fn calculate_steps(square: Square) -> u32 {
   (square.x.abs() + square.y.abs()) as u32
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let limit: usize = input.trim().parse::<usize>().unwrap();

    let mut grid: Vec<Square> = Vec::with_capacity(limit as usize);

    build_grid(&mut grid, limit);

    let target_square = grid[limit-1].clone();
//    println!("target_square = {:?}", target_square);
    println!("steps for {} = {}", limit, calculate_steps(target_square));
}
