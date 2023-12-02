use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()>{
    let path = Path::new("day2_part1_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut valid_game_sum = 0;
    for line in reader.lines()
    {
        let line = line?;
        let sub_line: Vec<&str> = line.split(':').collect();

        //parse out the game id
        let game_id = sub_line[0].split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        println!("{}", game_id);
        
        //parse out the marble count; looping through the games to find valid games
        let marble_counts: Vec<&str> = sub_line[1].split(';').collect();
        let mut game_is_valid = true;
        for marble_count in marble_counts
        {
            //get individual color counts
            let marble_count: Vec<&str> = marble_count.split(',').collect();
            for marble in marble_count
            {
                let marble = marble.trim();
                //get individual marble count
                let marble: Vec<&str> = marble.split(' ').collect();
                let marble_count = marble[0].parse::<i32>().unwrap();
                let marble_color = marble[1];
                
                match marble_color
                {
                    "red" => {
                        if marble_count > max_red {
                            println!("Invalid game: {}", game_id);
                            game_is_valid = false;
                            break;
                        }
                    },
                    "green" => {
                        if marble_count > max_green {
                            println!("Invalid game: {}", game_id);
                            game_is_valid = false;
                            break;
                        }
                    },
                    "blue" => {
                        if marble_count > max_blue {
                            println!("Invalid game: {}", game_id);
                            game_is_valid = false;
                            break;
                        }
                    },
                    _ => {
                        println!("Invalid game: {}", game_id);
                        break;
                    }
                }

            }
        }
        if game_is_valid == true 
        {
            println!("Valid game: {}", game_id);
            valid_game_sum += game_id;
        }
    }
    println!("Valid game sum: {}", valid_game_sum);
    Ok(())
}
