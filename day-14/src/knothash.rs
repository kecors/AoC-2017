#[derive(Debug)]
struct KnotHash {
    list: Vec<u32>,
    position: usize,
    skip: usize,
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
            skip: 0,
        }
    }

    fn twist(&mut self, length: usize) {
        //        println!("pre  twist: {:?}, {}", self, length);
        // - Reverse the order of that length of elements in the list,
        // starting with the element at the current position.
        let mut forward = self.position;
        let mut backward = (self.position + length - 1) % self.list.len();
        for _ in 0..(length / 2) {
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

pub fn make_hexadecimal_string(input: &String) -> String {
    let mut lengths: Vec<u8> = Vec::new();
    lengths.extend(input.as_bytes());
    lengths.extend(&[17, 31, 73, 47, 23]);

    let mut kh = KnotHash::new(256);
    for _ in 0..64 {
        for length in lengths.clone() {
            let p = length as usize;
            kh.twist(p);
        }
    }

    let dh = calculate_dense_hash(&kh.list);
    let hs = calculate_hexadecimal_string(&dh);
    return hs;
}

pub fn hexstring_to_binary(hexstring: &String) -> Vec<u32> {
    let mut result = Vec::new();

    for c in hexstring.chars() {
        match c {
            '0' => { result.extend(&[0,0,0,0]); },
            '1' => { result.extend(&[0,0,0,1]); },
            '2' => { result.extend(&[0,0,1,0]); },
            '3' => { result.extend(&[0,0,1,1]); },
            '4' => { result.extend(&[0,1,0,0]); },
            '5' => { result.extend(&[0,1,0,1]); },
            '6' => { result.extend(&[0,1,1,0]); },
            '7' => { result.extend(&[0,1,1,1]); },
            '8' => { result.extend(&[1,0,0,0]); },
            '9' => { result.extend(&[1,0,0,1]); },
            'a' => { result.extend(&[1,0,1,0]); },
            'b' => { result.extend(&[1,0,1,1]); },
            'c' => { result.extend(&[1,1,0,0]); },
            'd' => { result.extend(&[1,1,0,1]); },
            'e' => { result.extend(&[1,1,1,0]); },
            'f' => { result.extend(&[1,1,1,1]); },
            _   => { unimplemented!("hexstring_to_binary"); }
        }
    }

    result
}
