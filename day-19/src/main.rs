use std::io::{stdin, Read};
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct State {
    diagram: Vec<Vec<char>>,
    row: usize,
    column: usize,
    direction: Direction,
    letters: Vec<char>,
    steps: u32,
}

impl State {
    fn new(lines: Vec<&str>) -> State {
        let mut diagram: Vec<Vec<char>> = Vec::new();
        for line in lines.iter() {
            diagram.push(line.chars().collect());
        }
        let mut column: usize = 0;
        loop {
            if diagram[0][column] == '|' {
                break;
            }
            column += 1;
        }

        State {
            diagram: diagram,
            row: 0,
            column: column,
            direction: Direction::DOWN,
            letters: Vec::new(),
            steps: 0,
        }
    }

    fn go(&mut self) {
        loop {
            match self.direction {
                Direction::UP => {
                    self.row -= 1;
                }
                Direction::DOWN => {
                    self.row += 1;
                }
                Direction::LEFT => {
                    self.column -= 1;
                }
                Direction::RIGHT => {
                    self.column += 1;
                }
            }
            self.steps += 1;
            match self.diagram[self.row][self.column] {
                ' ' => {
                    println!("part 1: letters = {}", String::from_iter(&self.letters));
                    println!("part 2: took {} steps", self.steps);
                    break;
                }
                '+' => {
                    if self.direction == Direction::UP || self.direction == Direction::DOWN {
                        if self.diagram[self.row][self.column + 1] == ' ' {
                            self.direction = Direction::LEFT;
                        } else {
                            self.direction = Direction::RIGHT;
                        }
                    } else if self.direction == Direction::LEFT
                        || self.direction == Direction::RIGHT
                    {
                        if self.diagram[self.row + 1][self.column] == ' ' {
                            self.direction = Direction::UP;
                        } else {
                            self.direction = Direction::DOWN;
                        }
                    }
                    println!(
                        "[{:3}, {:3}] turn {:?}",
                        self.row, self.column, self.direction
                    );
                }
                c => {
                    if c.is_alphabetic() {
                        self.letters.push(c);
                        println!("[{:3}, {:3}] add letter {}", self.row, self.column, c);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let mut state = State::new(lines);

    state.go();
}
