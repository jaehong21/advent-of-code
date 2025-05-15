use std::fs::File;
use std::io::{self, BufRead, Result};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let nums: Vec<i32> = parts.iter().filter_map(|s| s.parse::<i32>().ok()).collect();

        let mut is_safe = true;

        let is_increasing = (nums[1] - nums[0]) > 0;
        if is_increasing {
            for i in 0..nums.len() - 1 {
                if nums[i] >= nums[i + 1] {
                    is_safe = false;
                    break;
                }
            }
        } else {
            for i in 0..nums.len() - 1 {
                if nums[i] <= nums[i + 1] {
                    is_safe = false;
                    break;
                }
            }
        }

        if !is_safe {
            continue;
        }

        for i in 0..nums.len() - 1 {
            let diff_abs = (nums[i] - nums[i + 1]).abs();
            if diff_abs < 1 || diff_abs > 3 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe_count += 1;
        }
    }

    println!("{}", safe_count);

    Ok(())
}
