//NOTE: brute force won't work for the given input as its would take 197 days to complete

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
    let mut step_count = 0;
    let mut all_end_in_Z = false;
    //let mut directions_progress = 0;
    while !all_end_in_Z 
    {
        //println!("Restarting directions {}" , directions_progress);
        //directions_progress += 1;
        for c in directions.chars()
        { 
            all_end_in_Z = true;
            for c_p_i in 0..current_positions.len()
            {
                println!("c_p_i: {}", c_p_i);
                let c_p = &current_positions[c_p_i];
                println!("c_p: {}", c_p);
                let current_step = steps.get(c_p).unwrap();
                if c == 'L'
                {
                    current_positions[c_p_i] = current_step.0.clone();
                    println!("Moving left to: {}", current_positions[c_p_i]);
                }
                else if c == 'R'
                {
                    current_positions[c_p_i] = current_step.1.clone();
                    println!("Moving right to: {}", current_positions[c_p_i]);
                }
                else
                {
                    panic!("Invalid direction: {}", c);
                }

                if all_end_in_Z && !current_positions[c_p_i].ends_with('Z')
                {
                    //println!("{} does not end in Z", current_positions[c_p_i]);
                    all_end_in_Z = false;
                }
                //else {
                    //println!("{} ends in Z", current_positions[c_p_i]);
                //}
            }
            step_count += 1;
            if all_end_in_Z
            {
                println!("All end in Z--Break!");
                break;
            }
        }
    }

    println!("Step count: {}", step_count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}