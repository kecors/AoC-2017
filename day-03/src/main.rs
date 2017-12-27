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
    sum:       Option<u64>
}

impl Square {
    fn initial() -> Square {
        Square { x: 0, y: 0, direction: Direction::RIGHT, sum: Some(1) }
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

    fn summable_squares(&self, position: u32) -> Vec<(i32,i32)> {
        let mut result : Vec<(i32,i32)> = Vec::new();

        match self.direction {
            Direction::RIGHT => {
                result.push((self.x - 1, self.y    ));
                result.push((self.x - 1, self.y + 1));
            },
            Direction::UP => {
                result.push((self.x,     self.y - 1));
                result.push((self.x - 1, self.y - 1));
            },
            Direction::LEFT => {
                result.push((self.x + 1, self.y    ));
                result.push((self.x + 1, self.y - 1));
            },
            Direction::DOWN => {
                result.push((self.x,     self.y + 1));
                result.push((self.x + 1, self.y + 1));
            }
        };
        if position == 0 { return result; }

        match self.direction {
            Direction::RIGHT => {
                result.push((self.x,     self.y + 1));
            },
            Direction::UP => {
                result.push((self.x - 1, self.y    ));
            },
            Direction::LEFT => {
                result.push((self.x,     self.y - 1));
            },
            Direction::DOWN => {
                result.push((self.x + 1, self.y    ));
            }
        };
        if position == 1 { return result; }
        
        match self.direction {
            Direction::RIGHT => {
                result.push((self.x + 1, self.y + 1));
            },
            Direction::UP => {
                result.push((self.x - 1, self.y + 1));
            },
            Direction::LEFT => {
                result.push((self.x - 1, self.y - 1));
            },
            Direction::DOWN => {
                result.push((self.x + 1, self.y - 1));
            }
        };
        return result;
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
        step += 1;

        // Calcuate sum here
        let ss = square.summable_squares(distance - step);
        let mut sum = 0;
//        println!("ss = {:?}", ss);
        for f in grid.iter().rev() {
            for g in ss.clone() {
                if f.x == g.0 && f.y == g.1 {
//                    println!("f = {:?}", f);
                    sum += f.sum.unwrap();
                }
            }
        }
        square.sum = Some(sum);
        if sum > (limit as u64) {
            println!("sum = {}", sum);
            std::process::exit(0);
        }

        // Turn the corner
        if step == distance {
            step = 0;
            if square.direction == Direction::UP || 
               square.direction == Direction::DOWN {
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

    println!("limit {}: steps = {}", limit, grid[limit-1].calculate_steps());
}
