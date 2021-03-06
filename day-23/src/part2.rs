use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Default)]
pub struct State {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
    g: i64,
    h: i64,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "a {} b {} c {} d {} e {} f {} g {} h {}",
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h
        )
    }
}

impl State {
    pub fn new() -> State {
        let mut state = State::default();
        state.a = 1;
        state
    }

    pub fn run(&mut self) {
        self.b = 79;
        self.c = self.b;
        if self.a != 0 {
            self.b *= 100;
            self.b += 100000;
            self.c = self.b;
            self.c += 17000;
        }
        'i09: loop {
            self.f = 1;
            self.d = 2;
            'i11: loop {
                self.e = 2;
                'i12: loop {
                    self.g = self.d;
                    self.g *= self.e;
                    self.g -= self.b;
                    if self.g == 0 {
                        self.f = 0;
                    }
                    self.e += 1;
                    self.g = self.e;
                    self.g -= self.b;
                    // The following optimization makes a huge difference
                    if self.b % self.d == 0 && self.g != 0 {
                        continue 'i12;
                    }
                    self.d += 1;
                    self.g = self.d;
                    self.g -= self.b;
                    if self.g != 0 {
                        break;
                    }
                    if self.f == 0 {
                        self.h += 1;
                    }
                    self.g = self.b;
                    self.g -= self.c;
                    if self.g == 0 {
                        println!("part 2: h = {}", self.h);
                        return;
                    } else {
                        break 'i11;
                    }
                }
            }
            self.b += 17;
        }
    }
}
