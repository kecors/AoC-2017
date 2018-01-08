use std::io::{stdin, Read};
use std::fmt::{Display, Formatter, Error};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "particle.pest"]
struct ParticleParser;

#[derive(Debug, Default)]
struct Coordinates {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug, Default)]
struct Particle {
    position: Coordinates,
    velocity: Coordinates,
    acceleration: Coordinates
}

impl Particle {
    fn new() -> Particle {
        Particle::default()
    }

    fn tick(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn manhattan_distance(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }
}

impl Display for Particle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
               self.position.x, self.position.y, self.position.z,
               self.velocity.x, self.velocity.y, self.velocity.z,
               self.acceleration.x, self.acceleration.y, self.acceleration.z)
    }
}

#[derive(Debug)]
struct State {
    particles: Vec<Particle>
}

impl State {
    fn new(particles: Vec<Particle>) -> State {
        State {
            particles
        }
    }

    fn tick(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.tick();
        }
    }

    fn closest(&self) {
        let (index, particle) = self.particles
                                    .iter()
                                    .enumerate()
                                    .min_by(|x, y|
                                        x.1.manhattan_distance()
                                         .cmp(&y.1.manhattan_distance())
                                    )
                                    .unwrap();
        println!("[{}] {}", index, particle);
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
            },
            Rule::py => {
                particle.position.y = text.parse().unwrap();
            },
            Rule::pz => {
                particle.position.z = text.parse().unwrap();
            },
            Rule::vx => {
                particle.velocity.x = text.parse().unwrap();
            },
            Rule::vy => {
                particle.velocity.y = text.parse().unwrap();
            },
            Rule::vz => {
                particle.velocity.z = text.parse().unwrap();
            },
            Rule::ax => {
                particle.acceleration.x = text.parse().unwrap();
            },
            Rule::ay => {
                particle.acceleration.y = text.parse().unwrap();
            },
            Rule::az => {
                particle.acceleration.z = text.parse().unwrap();
            },
            _ => { unimplemented!("parse_line"); }
        }
    }

    particle
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let particles: Vec<Particle> = input.lines()
                                        .map(|line| parse_line(line))
                                        .collect();

    let mut state = State::new(particles);

    // This arbitrarily selected number of repetitions yields
    // the correct answer
    for _ in 0..500 {
        state.closest();
        state.tick();
    }
}
