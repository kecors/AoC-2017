use std::io;

#[derive(Debug)]
struct KnotHash {
    list: Vec<usize>,
    position: usize,
    skip: usize
}

impl KnotHash {
    fn new(list_size: usize) -> KnotHash {
        let mut list = Vec::new();
        for j in 0..list_size {
            list.push(j);
        }
        KnotHash {
            list: list,
            position: 0,
            skip: 0
        }
    }

    fn twist(&mut self, length: usize) {
//        println!("pre  twist: {:?}, {}", self, length);
        // - Reverse the order of that length of elements in the list, 
        // starting with the element at the current position.
        let mut forward = self.position;
        let mut backward = (self.position + length - 1) % self.list.len();
        for _ in 0..(length/2) {
//            println!("forward = {}, backward = {}", forward, backward);
            self.list.swap(forward, backward);
            forward += 1;
            if forward == self.list.len() {
                forward = 0;
            }
            if backward == 0 {
                backward = self.list.len() - 1;
            } else {
                backward -= 1;
            }
        }

        // - Move the current position forward by that length plus 
        // the skip size.
        self.position = (self.position + length + self.skip) % self.list.len();

        // - Increase the skip size by one.
        self.skip += 1;
    }

    fn product_first_two(&self) -> usize {
        self.list[0] * self.list[1]
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let strs: Vec<&str> = input.trim()
                               .split(',')
                               .collect();
    let lengths: Vec<usize> = strs.iter()
                                .map(|x| x.parse::<usize>().unwrap())
                                .collect();
//    println!("lengths = {:?}", lengths);
    let mut kh = KnotHash::new(256);
    for length in lengths {
        kh.twist(length);
    }
//    println!("kh = {:?}", kh);
    println!("product first two = {}", kh.product_first_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut kh = KnotHash::new(5);
        for length in [3, 4, 1, 5].iter() {
            kh.twist(*length as usize);
        }
        println!("kh = {:?}", kh);
        assert_eq!(12, kh.product_first_two());
    }
}
