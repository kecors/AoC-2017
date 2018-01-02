use std::io;

#[derive(Debug)]
struct KnotHash {
    list: Vec<u32>,
    position: usize,
    skip: usize
}

impl KnotHash {
    fn new(list_size: usize) -> KnotHash {
        let mut list = Vec::new();
        for j in 0..list_size {
            list.push(j as u32);
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

    fn product_first_two(&self) -> u32 {
        self.list[0] * self.list[1]
    }
}

// Assuming spare hash has 256 elements
fn calculate_dense_hash(spare_hash: &Vec<u32>) -> Vec<u32> {
    let mut dense_hash: Vec<u32> = Vec::new();

    let mut sp = spare_hash.clone();
    for _ in 0..16 {
        let remainder = sp.split_off(16);
        let k = sp.pop().unwrap();
        let result = sp.iter().fold(k, |acc, &x| acc ^ x);
        dense_hash.push(result);
        sp = remainder;
    }

    dense_hash
}

// Assuming dense hash has 16 elements
fn calculate_hexadecimal_string(dense_hash: &Vec<u32>) -> String {
    let mut result = String::new();

    for value in dense_hash {
        let h = format!("{:02x}", value);
        result.push_str(h.as_str());
    }

    result
}

fn do_part1(input: &String) {
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

fn do_part2(input: &String) -> String {
//    let mut lengths = ascii(input.as_str());
    let mut lengths: Vec<u8> = Vec::new();
    lengths.extend(input.as_bytes());
    lengths.extend(&[17,31,73,47,23]);
//    println!("lengths = {:?}", lengths);

    let mut kh = KnotHash::new(256);
    for _ in 0..64 {
        for length in lengths.clone() {
            let p = length as usize;
            kh.twist(p);
        }
    }
//    println!("kh = {:?}", kh);
//    println!("product first two = {}", kh.product_first_two());

    let dh = calculate_dense_hash(&kh.list);
//    println!("dh = {:?}", dh);
    let hs = calculate_hexadecimal_string(&dh);
    return hs
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    do_part1(&input);

    let result = do_part2(&input);
    println!("part 2 result = {}", result);
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

    #[test]
    fn try_part2_1() {
        let x = "".to_string();
        let z = do_part2(&x);
        assert_eq!(z, "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn try_part2_2() {
        let x = "AoC 2017".to_string();
        let z = do_part2(&x);
        assert_eq!(z, "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn try_part2_3() {
        let x = "1,2,3".to_string();
        let z = do_part2(&x);
        assert_eq!(z, "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn try_part2_4() {
        let x = "1,2,4".to_string();
        let z = do_part2(&x);
        assert_eq!(z, "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
