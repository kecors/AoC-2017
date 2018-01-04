use std::io::{stdin, Read};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    DOWN
}

#[derive(Debug)]
struct Layer {
    depth: u32,
    range: u32,
    scanner: u32,
    direction: Direction
}

impl Layer {
    fn new(depth: u32, range: u32) -> Layer {
        Layer {
            depth: depth, 
            range: range,
            scanner: 0,
            direction: Direction::DOWN
        }
    }

    fn tick(&mut self) {
        match self.direction {
            Direction::DOWN => {
                self.scanner += 1;
                if self.scanner == self.range-1 {
                    self.direction = Direction::UP;
                }
            },
            Direction::UP => {
                self.scanner -= 1;
                if self.scanner == 0 {
                    self.direction = Direction::DOWN;
                }
            }
        }
    }
}

#[derive(Debug)]
struct State {
    layer_hm: HashMap<u32, Layer>,
    severity: u32
}

impl State {
    fn new(layers: Vec<Layer>) -> State {
        let mut layer_hm = HashMap::new();

        for layer in layers {
            layer_hm.insert(layer.depth, layer);
        }

        State {
            layer_hm: layer_hm,
            severity: 0
        }
    }

    fn maximum_depth(&self) -> u32 {
        let mut maximum_depth: u32 = 0;
        for depth in self.layer_hm.keys() {
            if *depth > maximum_depth {
                maximum_depth = *depth;
            }
        }
        maximum_depth
    }

    fn travel(&mut self) {
        for depth in 0..self.maximum_depth() {
            let mut severity: u32 = 0;
            if let Some(layer) = self.layer_hm.get(&depth) {
                if layer.scanner == 0 {
                    severity = layer.depth * layer.range;
                }
            }
            self.severity += severity;
            for layer in self.layer_hm.values_mut() {
                layer.tick();
            }
        }
    }
}

fn parse_line(line: &str) -> Layer {
   let mut values = line.split(": ");
   let depth: u32 = values.next().unwrap().parse().unwrap();
   let range: u32 = values.next().unwrap().parse().unwrap();

   Layer::new(depth, range)
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();
//    println!("input = {:#?}", input);

    let layers: Vec<Layer> = input.lines().map(|line| parse_line(line)).collect();
//    println!("layers = {:?}", layers);

    let mut state = State::new(layers);
    state.travel();
    println!("severity = {}", state.severity);
}
