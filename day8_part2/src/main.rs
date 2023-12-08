use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use std::collections::HashMap;
fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day8_part2_input.txt");
    //let path = Path::new("day8_part2_test_input.txt");
    let file = File::open(&path)?;
    let mut reader = io::BufReader::new(file);

    //read first line for the directions & initialize to empty string
    let mut directions = String::new();
    reader.read_line(&mut directions)?;
    let directions = directions.trim();
    //read one more line to skip the blank line
    let mut blank_line = String::new();
    reader.read_line(&mut blank_line)?;

    let mut steps: HashMap<String, (String, String)> = HashMap::new();
    let mut current_positions: Vec<String> = Vec::new();
    for line in reader.lines()
    {
        let line = line?;
        let line_parts = line.split([' ', '=', '(', ')', ',']).collect::<Vec<&str>>();
        println!("line_parts: {:?}", line_parts);
        steps.insert(line_parts[0].to_string(), (line_parts[4].to_string(), line_parts[6].to_string()));
        if line_parts[0].ends_with('A')
        {
            println!("Adding {} to current_positions", line_parts[0]);
            current_positions.push(line_parts[0].to_string());
        }
    } 
    let mut step_counts: Vec<u64> = Vec::new(); 
    for c_p in &current_positions
    {
        let mut current_position = c_p; 
        let mut step_count = 0;
        while !current_position.ends_with('Z')
        {
            for c in directions.chars()
            {
                let current_step = steps.get(current_position).unwrap();
                if c == 'L'
                {
                    current_position = &current_step.0;
                }
                else if c == 'R'
                {
                    current_position = &current_step.1;
                }
                else
                {
                    panic!("Invalid direction: {}", c);
                }
                step_count += 1;

                //ensure we didn't get there early
                if current_position.ends_with('Z')
                {
                    break;
                }
            }
        }
        println!("Step count: {}", step_count);
        step_counts.push(step_count);
    }
    //find least common multiple of all step counts
    let mut total_step_count: u64 = 1;
    
    for s in step_counts
    {
        total_step_count = num::integer::lcm(total_step_count, s);
    }

    println!("Total Step count: {}", total_step_count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}