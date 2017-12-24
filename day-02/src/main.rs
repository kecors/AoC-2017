use std::io::{stdin, Read};

fn process_row_part_1(row: &Vec<u32>) -> u32 {
    let mut min: u32 = row[0];
    let mut max: u32 = row[0];

    for x in 1..row.len() {
        if row[x] < min { min = row[x]; }
        if row[x] > max { max = row[x]; }
    }

    max - min
}

fn process_row_part_2(row: &Vec<u32>) -> u32 {
    let mut result = 0;

    for x in row {
        let row_clone = row.clone();
        for y in row_clone {
            if *x == y { continue; }
            if *x % y == 0 { result = *x / y; break; }
        }
    }

    result
}

fn process_rows<F>(rows: &Vec<Vec<u32>>, f: F) -> u32
    where F: Fn(&Vec<u32>) -> u32 {
    let mut sum: u32 = 0;

    for row in rows {
        sum += f(&row);
    }

    sum
}

fn parse_line(line: &str) -> Vec<u32> {
   let digits: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    digits
}

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();
//    println!("input = {:#?}", input);

    let rows: Vec<Vec<u32>> = input.lines().map(|line| parse_line(line)).collect();
//    println!("rows = {:?}", rows);

    let result = process_rows(&rows, process_row_part_1);
    println!("result, part 1 = {}", result);

    let result = process_rows(&rows, process_row_part_2);
    println!("result, part 2 = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_process_row_provided_tests() {
        assert_eq!(8, process_row_part_1(&vec![5,1,9,5]));
        assert_eq!(4, process_row_part_1(&vec![7,5,3]));
        assert_eq!(6, process_row_part_1(&vec![2,4,6,8]));
    }

    #[test]
    fn part_2_process_row_provided_tests() {
        assert_eq!(4, process_row_part_2(&vec![5,9,2,8]));
        assert_eq!(3, process_row_part_2(&vec![9,4,7,3]));
        assert_eq!(2, process_row_part_2(&vec![3,8,6,5]));
    }
}
