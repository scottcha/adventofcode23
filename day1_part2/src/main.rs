use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() -> io::Result<()> {
    let path = Path::new("day1_part2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let number_string_list = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let number_int_list = vec!['1', '2','3', '4', '5', '6', '7', '8', '9'];
    let mut total_sum = 0;
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        let mut numbers = Vec::new();

        // Iterate over the line by index
        for (i, c) in line.chars().enumerate() 
        {
            if c.is_numeric() {
                // Store the digits in an array
                numbers.push(c);
            }
            else {
                //check if the current string starting from index i is in the list
                //iterate through number_list
                for i_number_string in 0..number_string_list.len()
                {
                    let num_string = number_string_list[i_number_string];
                    //println!("num_string: {}", num_string);
                    //create a slice of line from i to size of num_string
                    //catch out of bounds error
                    if i + num_string.len() > line.len() {
                        continue;
                    }
                    let slice = &line[i..i+num_string.len()];
                    //println!("slice: {}", slice);
                    //compare by value to num_string
                    if slice == num_string {
                        //if true, push the number corresponding to the index of num_string
                        numbers.push(number_int_list[i_number_string]);
                        break;
                    }
                }
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