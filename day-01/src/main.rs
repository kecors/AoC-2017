use std::io;

fn summer1(digits: &[u32]) -> u32 {
    let mut sum = 0;

    // Proceed forward through the list of digits
    for j in 0..(digits.len()-1) {
        if digits[j] == digits[j+1] {
            sum += digits[j];
        }
    }

    // Check if last digit matches first
    if digits[digits.len()-1] == digits[0] {
        sum += digits[0];
    }

    sum
}

fn summer2(digits: &[u32]) -> u32 {
    let mut sum = 0;

    let half_len = digits.len() / 2;

    // Proceed forward through the list of digits
    for j in 0..half_len {
        if digits[j] == digits[j+half_len] {
            sum += digits[j];
        }
    }

    sum + sum
}

fn parser(input: &str) -> Vec<u32> {
    // Convert input into vector of u32s; panic on invalid input
    let digits: Vec<u32> = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();

    digits
}

fn main() {
    let mut input = String::new();

    // Read stdin
    io::stdin().read_line(&mut input).unwrap();
//    println!("input = {:?}", input);

    let digits = parser(&input);
//    println!("digits = {:?}", digits);

    let result_part_1 = summer1(&digits);
    println!("result, part 1 = {}", result_part_1);

    let result_part_2 = summer2(&digits);
    println!("result, part 2 = {}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summer1_provided_tests() {
        assert_eq!(3, summer1(&vec![1,1,2,2]));
        assert_eq!(4, summer1(&vec![1,1,1,1]));
        assert_eq!(0, summer1(&vec![1,2,3,4]));
        assert_eq!(9, summer1(&vec![9,1,2,1,2,1,2,9]));
    }

    #[test]
    fn summer1_other_tests() {
        assert_eq!(23, summer1(&vec![1,2,2,3,4,5,5,6,7,8,8,8,9,0]));
    }

    #[test]
    fn summer2_provided_tests() {
        assert_eq!(6, summer2(&vec![1,2,1,2]));
        assert_eq!(0, summer2(&vec![1,2,2,1]));
        assert_eq!(4, summer2(&vec![1,2,3,4,2,5]));
        assert_eq!(12, summer2(&vec![1,2,3,1,2,3]));
        assert_eq!(4, summer2(&vec![1,2,1,3,1,4,1,5]));
    }

    #[test]
    fn parser_provided_tests() {
        assert_eq!(vec![1,1,2,2], parser(&"1122".to_string()));
        assert_eq!(vec![1,1,1,1], parser(&"1111".to_string()));
        assert_eq!(vec![1,2,3,4], parser(&"1234".to_string()));
        assert_eq!(vec![9,1,2,1,2,1,2,9], parser(&"91212129".to_string()));
    }

    #[test]
    fn parser_other_tests() {
        assert_eq!(vec![1,2,2,3,4,5,5,6,7,8,8,8,9,0], parser(&"12234556788890".to_string()));
    }
}
