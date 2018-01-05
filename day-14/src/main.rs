use std::io;
mod knothash;

fn main() {
    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();

    let mut bits_total: u32 = 0;
    for j in 0..128 {
        let input: String = format!("{}-{}", key.trim(), j);
        let result: String = knothash::make_hexadecimal_string(&input);
//        println!("{:12} -> {}", input, result);
        let bits_vec = knothash::hexstring_to_binary(&result);
        let bits_sum: u32 = bits_vec.iter().sum();
//        println!("bits_vec = {:?}, sum = {}", bits_vec, bits_sum);
        bits_total += bits_sum;
    }
    println!("bits_total = {}", bits_total);
}
