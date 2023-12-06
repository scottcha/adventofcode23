use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day6_part2_input.txt");
    //let path = Path::new("day6_part1_test_input.txt");
    let file = File::open(&path)?;
    let mut reader = io::BufReader::new(file);
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    //read the two lines
    {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        println!("*line_parts: {:?}", line_parts);
        for i in 1..line_parts.len()
        {
            times.push(line_parts[i].parse::<u64>().unwrap());    
        }
    } 

    {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        println!("line_parts: {:?}", line_parts);
        for i in 1..line_parts.len()
        {
            distances.push(line_parts[i].parse::<u64>().unwrap());    
        }
    }
    let mut possible_product = 1; 
    for i in 0..times.len()
    {
        let t_i = times[i];
        let d_i = distances[i];
        let hold_times = (1..t_i)
            .filter(|&h| d_i < h * t_i - h * h) //satify the equation d < h(t-h) => d < h * t - h * h
            .count();
        
        println!("hold_times: {}", hold_times);
        possible_product *= hold_times;
    }
    println!("possible_product: {}", possible_product);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
