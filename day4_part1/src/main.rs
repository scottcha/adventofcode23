use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() -> io::Result<()>{
    let path = Path::new("day4_part1_input.txt");
    //let path = Path::new("day3_part2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_score: i32 = 0;
    for line in reader.lines()
    {
        let line = line?;
        let line_parts: Vec<&str> = line.split(&[':','|']).collect();
        let winning_numbers = line_parts[1].trim().split_whitespace().collect::<Vec<&str>>();
        let my_numbers = line_parts[2].trim().split_whitespace().collect::<Vec<&str>>();

        println!("winning_numbers: {:?}", winning_numbers);
        println!("my_numbers: {:?}", my_numbers);
        
        let mut set = HashSet::new();
        for number in winning_numbers
        {
            set.insert(number.parse::<i32>().unwrap());
        }

        let mut current_game_score = 0;
        for number in my_numbers
        {
            let inumber = number.parse::<i32>().unwrap();
            if set.contains(&inumber)
            {
                if current_game_score == 0
                {
                    current_game_score = 1;
                }
                else
                {
                    current_game_score *= 2;
                }
            }
        }
        total_score += current_game_score;
        println!("current_game_score: {}", total_score); 
    }
    Ok(())
}