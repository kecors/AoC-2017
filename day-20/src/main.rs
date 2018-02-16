use std::io::{stdin, Read};
use std::fmt::{Display, Error, Formatter};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "particle.pest"]
struct ParticleParser;

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
struct Coordinates {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Default, Clone)]
struct Particle {
    position: Coordinates,
    velocity: Coordinates,
    acceleration: Coordinates,
}

impl Particle {
    fn new() -> Particle {
        Particle::default()
    }

    fn tick(&mut self) -> Coordinates {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;

        self.position.clone()
    }

    fn manhattan_distance(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }
}

impl Display for Particle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z,
            self.acceleration.x,
            self.acceleration.y,
            self.acceleration.z
        )
    }
}

#[derive(Debug)]
struct State {
    particles: Vec<Particle>,
    collision_detector: HashMap<Coordinates, Vec<usize>>,
}

impl State {
    fn new(particles: Vec<Particle>) -> State {
        State {
            particles: particles,
            collision_detector: HashMap::new(),
        }
    }

    fn tick(&mut self) -> usize {
        self.collision_detector = HashMap::new();

        for (index, particle) in self.particles.iter_mut().enumerate() {
            let position = particle.tick();

            match self.collision_detector.entry(position) {
                Entry::Vacant(vacant) => {
                    let mut indices = Vec::new();
                    indices.push(index);
                    vacant.insert(indices);
                }
                Entry::Occupied(mut occupied) => {
                    occupied.get_mut().push(index);
                }
            }
        }

        // Check for collisions and remove affected particles
        let mut collided: Vec<usize> = Vec::new();
        for indices in self.collision_detector.values() {
            if indices.len() > 1 {
                collided.extend(indices);
            }
        }
        collided.sort();
        collided.reverse();
        for index in collided.iter() {
            self.particles.remove(*index);
        }
        collided.len()
    }

    fn closest(&self) {
        let (index, particle) = self.particles
            .iter()
            .enumerate()
            .min_by(|x, y| x.1.manhattan_distance().cmp(&y.1.manhattan_distance()))
            .unwrap();
        println!("[{}] {}", index, particle);
    }

    fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

fn parse_line(line: &str) -> Particle {
    let pairs = ParticleParser::parse_str(Rule::particle, line).unwrap_or_else(|e| panic!("{}", e));

    let mut particle = Particle::new();

    for pair in pairs {
        let rule = pair.as_rule();
        let text = pair.clone().into_span().as_str().to_string();
        match rule {
            Rule::px => {
                particle.position.x = text.parse().unwrap();
            }
            Rule::py => {
                particle.position.y = text.parse().unwrap();
            }
            Rule::pz => {
                particle.position.z = text.parse().unwrap();
            }
            Rule::vx => {
                particle.velocity.x = text.parse().unwrap();
            }
            Rule::vy => {
                particle.velocity.y = text.parse().unwrap();
            }
            Rule::vz => {
                particle.velocity.z = text.parse().unwrap();
            }
            Rule::ax => {
                particle.acceleration.x = text.parse().unwrap();
            }
            Rule::ay => {
                particle.acceleration.y = text.parse().unwrap();
            }
            Rule::az => {
                particle.acceleration.z = text.parse().unwrap();
            }
            _ => {
                unimplemented!("parse_line");
            }
        }
    }

    particle
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let particles: Vec<Particle> = input.lines().map(|line| parse_line(line)).collect();

    let mut state = State::new(particles);

    // This arbitrarily selected number of repetitions yields
    // the correct answer for part 1 and for part 2
    for x in 0..500 {
        print!("{:4} ", x);
        state.closest();
        let collided_count = state.tick();
        if collided_count > 0 {
            println!(
                "---- {} collided particles removed; {} remain",
                collided_count,
                state.particle_count()
            );
        }
    }
    println!("part 2: particle count = {:?}", state.particle_count());
}
