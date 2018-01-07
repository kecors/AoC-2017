#[derive(Debug)]
struct Spinlock {
    buffer: Vec<u32>,
    position: usize,
    steps: usize
}

impl Spinlock {
    fn new(steps: usize) -> Spinlock {
        let mut buffer = Vec::new();
        buffer.push(0);

        let position = 0;

        Spinlock {
            buffer,
            position,
            steps
        }
    }

    fn insert(&mut self, value: u32) {
        self.position = 1 + (self.position + self.steps) % self.buffer.len();
        self.buffer.insert(self.position, value);
    }
}

fn main() {
    let steps: usize = 337;

    let mut spinlock = Spinlock::new(steps);

    for value in 1..2018 {
        spinlock.insert(value);
    }

    for index in 0..spinlock.buffer.len() {
        if spinlock.buffer[index] == 2017 {
            println!("part 1: {}", spinlock.buffer[index+1]);
            break;
        }
    }
}
