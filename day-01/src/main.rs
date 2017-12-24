use std::io;

fn summer(digits: &Vec<u32>) -> u32 {
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

fn parser(input: &String) -> Vec<u32> {
    // Convert input into vector of u32s; panic on invalid input
    let digits: Vec<u32> = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();

    digits
}

fn main() {
    let mut input = String::new();

    // Read stdin
    io::stdin().read_line(&mut input).unwrap();

    let digits = parser(&input);
    let result = summer(&digits);

//    println!("input = {:?}", input);
//    println!("digits = {:?}", digits);

    println!("result = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summer_provided_tests() {
        assert_eq!(3, summer(&vec![1,1,2,2]));
        assert_eq!(4, summer(&vec![1,1,1,1]));
        assert_eq!(0, summer(&vec![1,2,3,4]));
        assert_eq!(9, summer(&vec![9,1,2,1,2,1,2,9]));
    }

    #[test]
    fn summer_other_tests() {
        assert_eq!(23, summer(&vec![1,2,2,3,4,5,5,6,7,8,8,8,9,0]));
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
