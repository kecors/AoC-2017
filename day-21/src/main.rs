use std::io::{stdin, Read};
use std::fmt::{Display, Formatter, Error};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn new(string: &str) -> Grid {
        let mut outer_vec = Vec::new();
        let lines = string.split('/');
        for line in lines {
            let mut inner_vec = Vec::new();
            for c in line.chars() {
                inner_vec.push(if c == '#' { true } else { false });
            }
            outer_vec.push(inner_vec);
        }

        Grid(outer_vec)
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn rotate(&self) -> Grid {
        let mut outer_vec = Vec::new();
        let size = self.0.len();
        for x in 0..size {
            outer_vec.push(Vec::new());
            for y in (0..size).rev() {
                outer_vec[x].push(self.0[y][x]);
            }
        }
        Grid(outer_vec)
    }

    fn flip(&self) -> Grid {
        let mut outer_vec = Vec::new();
        let size = self.0.len();
        for x in 0..size {
            outer_vec.push(Vec::new());
            for y in 0..size {
                outer_vec[x].push(self.0[size-x-1][y]);
            }
        }
        Grid(outer_vec)
    }

    fn generate_transforms(&self) -> Vec<Grid> {
        let mut transforms: HashSet<Grid> = HashSet::new();
        let rotate_0 = self.clone();
        let rotate_1 = rotate_0.rotate();
        let rotate_2 = rotate_1.rotate();
        let rotate_3 = rotate_2.rotate();
        let flipped_0 = self.flip();
        let flipped_1 = flipped_0.rotate();
        let flipped_2 = flipped_1.rotate();
        let flipped_3 = flipped_2.rotate();
        transforms.insert(rotate_0);
        transforms.insert(rotate_1);
        transforms.insert(rotate_2);
        transforms.insert(rotate_3);
        transforms.insert(flipped_0);
        transforms.insert(flipped_1);
        transforms.insert(flipped_2);
        transforms.insert(flipped_3);
        transforms.into_iter().collect()
    }

    fn subdivide(&self) -> Vec<Grid> {
        let mut squares: Vec<Grid> = Vec::new();
        let mut subvecs: Vec<Vec<Vec<bool>>> = Vec::new();
        let chunk_size = if self.0.len() % 2 == 0 { 2 } else { 3 };
        let chunk_count = self.0.len() / chunk_size;
        for _ in 0..chunk_count.pow(2) {
            subvecs.push(Vec::new());
        }
        let row_chunks = self.0.chunks(chunk_size);
        for (rc_index,rc_content) in row_chunks.enumerate() {
            for row in rc_content {
                let column_chunks = row.chunks(chunk_size);
                for (cc_index,cc_content) in column_chunks.enumerate() {
                    let offset = rc_index * chunk_count + cc_index;
                    subvecs[offset].push(cc_content.to_vec());
                }
            }
        }
        for v in subvecs {
            squares.push(Grid(v));
        }
        squares
    }

    fn combine(squares: Vec<Grid>) -> Grid {
        let mut result: Vec<Vec<bool>> = Vec::new();
        let length = (squares.len() as f32).sqrt() as usize;
        let size = squares[0].0.len();
        for _ in 0..length*size {
            result.push(Vec::new());
        }
        for x in 0..length {
            for y in 0..length {
                let index = x * length + y;
                for (j,subvec) in squares[index].0.iter().enumerate() {
                    let k = x * size + j;
                    result[k].extend(subvec);
                }
            }
        }
        Grid(result)
    }

    fn pixels_on_count(&self) -> u32 {
        let mut result: u32 = 0;
        for subvec in self.0.iter() {
            for value in subvec.iter() {
                if *value == true {
                    result += 1;
                }
            }
        }
        result
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut string = String::new();
        for inner_vec in self.0.iter() {
            string.push(' ');
            for item in inner_vec {
                string.push(if *item == true { '#' } else { '.' });
            }
            string.push('\n');
        }
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
struct State {
    image: Grid,
    rules: HashMap<Grid, Grid>
}

impl State {
    fn new(image: &str) -> State {
        State {
            image: Grid::new(image),
            rules: HashMap::new()
        }
    }

    fn add_rule(&mut self, input: Grid, output: Grid) {
        self.rules.insert(input, output);
    }

    fn replace(&self, grid: &Grid) -> Grid {
        for v in grid.generate_transforms() {
            if let Some(x) = self.rules.get(&v) {
                return x.clone();
            }
        }
        Grid(Vec::new())
    }

    fn iterate(&mut self) {
        let squares: Vec<Grid> = self.image.subdivide();
        let s2 = squares.iter().map(|x| self.replace(x)).collect();
        self.image = Grid::combine(s2);
    }

    fn display_image(&self) {
        print_border("!", self.image.size());
        print!("{}", self.image);
        print_border("~", self.image.size());
    }

    fn display_rules(&self) {
        for (k,v) in self.rules.iter() {
            print_border("-", v.size());
            print!("{}=>\n{}", k, v);
            print_border("=", v.size());
        }
    }

    fn pixels_on_count(&self) -> u32 {
        self.image.pixels_on_count()
    }
}

fn print_border(content: &str, repetitions: usize) {
    print!(" ");
    for _ in 0..repetitions {
        print!("{}", content);
    }
    println!("");
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(".#./..#/###");

    for line in input.lines() {
        let mut patterns = line.split(" => ");
        let input_pattern = Grid::new(patterns.next().unwrap());
        let output_pattern = Grid::new(patterns.next().unwrap());
        state.add_rule(input_pattern, output_pattern);
    }

    state.display_image();
    for _ in 0..5 {
        state.iterate();
        state.display_image();
    }
    println!("part 1: pixels on = {}", state.pixels_on_count());
}
