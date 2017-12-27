//
// Advent of Code 2017, Day 3
//
// To run part 1:
//     cat puzzle-input.txt | cargo run
//
// To run part 2:
//     cat puzzle-input.txt | cargo run -- features part2
//
// Key Observations:
//
// - The spiral is mapped on a two dimensional grid. Each square has an x
//   and a y value, in relation to the origin at square 1.
//
// - The spiral builds starting as follows: 1 right, 1 up, 2 left, 2 down,
//   3 right, 3 up, four left, 4 down... The code in Spiral::create_grid()
//   marked "Turn the corner" continues this pattern.
//
// - The sum for a square can use 2, 3 or 4 previous values, depending on
//   how soon the spiral will turn. Square::summable_squares() builds a 
//   list of these adjacent values. Spiral::calculate_sum() searches 
//   backward through the grid vector, matching on grid position and adding
//   up the new sum.
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
    sum:       Option<u32>
}

impl Square {
    fn initial() -> Square {
        Square { x: 0, y: 0, sum: Some(1) }
    }

    fn create_next(&self, direction: Direction) -> Square {
        let (x, y) : (i32, i32) = match direction {
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

        Square { x: x, y: y, sum: None }
    }

    #[cfg(feature="part2")]
    fn summable_squares(&self, direction: Direction, position: u32) -> Vec<(i32,i32)> {
        let mut result : Vec<(i32,i32)> = Vec::new();

        match direction {
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

        match direction {
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
        
        match direction {
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

    fn calculate_steps(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

struct Spiral {
    limit:     usize,
    grid:      Vec<Square>,
    direction: Direction,
    distance:  u32,
    step:      u32
}

impl Spiral {
    fn new(limit: usize) -> Spiral {
        Spiral {
            limit:     limit,
            grid:      Vec::with_capacity(limit),
            direction: Direction::RIGHT,
            distance:  1,
            step:      0
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::RIGHT => Direction::UP,
            Direction::UP    => Direction::LEFT,
            Direction::LEFT  => Direction::DOWN,
            Direction::DOWN  => Direction::RIGHT
        }
    }

    fn build_grid(&mut self) {
        let mut square = Square::initial();
        loop {
            let next_square = square.create_next(self.direction);
            self.grid.push(square);

            square = next_square;
            self.step += 1;

            self.calculate_sum(&mut square);

            // Turn the corner
            if self.step == self.distance {
                self.step = 0;
                if self.direction == Direction::UP || 
                   self.direction == Direction::DOWN {
                    self.distance += 1;
                }
                self.turn();
             }

             if self.grid.len() == self.limit { break; }
        }
    }

    #[cfg(feature="part2")]
    fn calculate_sum(&self, square: &mut Square) {
        let summable_squares = square.summable_squares(self.direction, self.distance - self.step);
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
