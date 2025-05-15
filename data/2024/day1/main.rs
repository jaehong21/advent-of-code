use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(left_num), Ok(right_num)) =
                (parts[0].parse::<i32>(), parts[1].parse::<i32>())
            {
                left_vec.push(left_num);
                right_vec.push(right_num);
            }
        }
    }

    left_vec.sort();
    right_vec.sort();

    let mut total = 0;
    for i in 0..left_vec.len() {
        let diff = (left_vec[i] - right_vec[i]).abs();
        total += diff;
    }

    println!("{}", total);
    Ok(())
}
