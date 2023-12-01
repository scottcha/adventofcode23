use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("day1_part1_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_sum = 0;
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                // Store the digits in an array
                numbers.push(c);
            }
        }
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        // Concat char first and last
        let mut concat = String::new();
        concat.push(first);
        concat.push(last);

        println!("subsum: {}", concat);

        // Update total_sum
        total_sum += concat.parse::<i32>().unwrap();
    }
    
    println!("Total sum: {}", total_sum);
    Ok(())
}