//
// To run part 1:
//     cat puzzle-input.txt | cargo run
//
// To run part 2:
//     cat puzzle-input.txt | cargo run -- features part2
//
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

    #[cfg(feature="part2")]
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

struct Spiral {
    limit:    usize,
    grid:     Vec<Square>,
    distance: u32,
    step:     u32
}

impl Spiral {
    fn new(limit: usize) -> Spiral {
        Spiral {
            limit:    limit,
            grid:     Vec::with_capacity(limit),
            distance: 1,
            step:     0
        }
    }

    fn build_grid(&mut self) {
        let mut square = Square::initial();
        loop {
            let next_square = square.create_next();
            self.grid.push(square);

            square = next_square;
            self.step += 1;

            self.calculate_sum(&mut square);

            // Turn the corner
            if self.step == self.distance {
                self.step = 0;
                if square.direction == Direction::UP || 
                   square.direction == Direction::DOWN {
                    self.distance += 1;
                }
                square.turn();
             }

             if self.grid.len() == self.limit { break; }
        }
    }

    #[cfg(feature="part2")]
    fn calculate_sum(&self, square: &mut Square) {
        let summable_squares = square.summable_squares(self.distance - self.step);
        let mut sum = 0;
        for ss in summable_squares {
            for q in self.grid.iter().rev() {
                if ss.0 == q.x && ss.1 == q.y {
                    sum += q.sum.unwrap();
                    break;
                }
            }
        }
        square.sum = Some(sum);
        if sum > (self.limit as u32) {
            println!("limit {}: part 2 - sum = {}", self.limit, sum);
            std::process::exit(0);
        }
    }

    #[cfg(not(feature="part2"))]
    fn calculate_sum(&self, _square: &mut Square) {
    }

    fn print_part_1_result(&self) {
        println!("limit {}: part 1 - steps = {}", 
                 self.limit, self.grid[self.limit-1].calculate_steps());
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let limit: usize = input.trim().parse::<usize>().unwrap();

    let mut spiral = Spiral::new(limit);
    spiral.build_grid();
    spiral.print_part_1_result();
}
