use std::io::{stdin, Read};

fn process_row(row: &Vec<u32>) -> u32 {
    let mut min: u32 = row[0];
    let mut max: u32 = row[0];

    for x in 1..row.len() {
        if row[x] < min { min = row[x]; }
        if row[x] > max { max = row[x]; }
    }

    max - min
}

fn process_rows(rows: &Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;

    for row in rows {
        sum += process_row(&row);
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

    let result = process_rows(&rows);
    println!("result = {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_row_provided_tests() {
        assert_eq!(8, process_row(&vec![5,1,9,5]));
        assert_eq!(4, process_row(&vec![7,5,3]));
        assert_eq!(6, process_row(&vec![2,4,6,8]));
    }
}
