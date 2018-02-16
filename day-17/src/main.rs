#[derive(Debug)]
struct Spinlock {
    buffer: Vec<u64>,
    position: usize,
    steps: usize,
}

impl Spinlock {
    fn new(steps: usize) -> Spinlock {
        let mut buffer = Vec::new();
        buffer.push(0);

        let position = 0;

        Spinlock {
            buffer,
            position,
            steps,
        }
    }

    fn insert(&mut self, value: u64) {
        self.position = 1 + (self.position + self.steps) % self.buffer.len();
        self.buffer.insert(self.position, value);
    }

    //
    // Customized function for part 2: since we only care about
    // values at position 1, we can push other values onto the end.
    // For Vec, push() is much faster than insert().
    //
    fn insert_or_push(&mut self, value: u64) {
        self.position = 1 + (self.position + self.steps) % self.buffer.len();
        if self.position == 1 {
            self.buffer.insert(self.position, value);
        } else {
            self.buffer.push(value);
        }
    }
}

fn do_part1() {
    let mut spinlock = Spinlock::new(337);

    for value in 1..2018 {
        spinlock.insert(value);
    }

    for index in 0..spinlock.buffer.len() {
        if spinlock.buffer[index] == 2017 {
            println!("part 1: {}", spinlock.buffer[index + 1]);
            break;
        }
    }
}

fn do_part2() {
    let mut spinlock = Spinlock::new(337);

    for value in 1..50000000 {
        spinlock.insert_or_push(value);
    }

    for index in 0..spinlock.buffer.len() {
        if spinlock.buffer[index] == 0 {
            println!("part 2: {}", spinlock.buffer[index + 1]);
            break;
        }
    }
}

fn main() {
    do_part1();
    do_part2();
}
