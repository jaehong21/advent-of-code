use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Result};

fn main() -> Result<()> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total_sum = 0;

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        for cap in re.captures_iter(&line) {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            total_sum += x * y;
        }
    }

    println!("{}", total_sum);

    Ok(())
}
