use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()>{
    let path = Path::new("day2_part2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

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
        let mut game_sub_powers: Vec<i32> = Vec::new();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for marble_count in marble_counts
        {
            //get individual color counts
            let marble_count: Vec<&str> = marble_count.split(',').collect();
            for marble in marble_count
            {
                let marble = marble.trim();
                //get individual marble count
                let marble: Vec<&str> = marble.split(' ').collect();
                let single_marble_count = marble[0].parse::<i32>().unwrap();
                let single_marble_color = marble[1];
                
                match single_marble_color
                {
                    "red" => {
                        if single_marble_count > max_red {
                            max_red = single_marble_count;
                        }
                    },
                    "green" => {
                        if single_marble_count > max_green {
                            max_green = single_marble_count;
                        }
                    },
                    "blue" => {
                        if single_marble_count > max_blue {
                            max_blue = single_marble_count;
                        }
                    },
                    _ => {
                        println!("Invalid game: {}", game_id);
                        break;
                    }
                }
            }
        }
        game_sub_powers.push(max_red * max_green * max_blue);
        //multiply the sub_powers together
        let mut game_power = 1;
        for sub_power in game_sub_powers
        {
            game_power *= sub_power;
        }
        println!("Game power: {}", game_power);
        valid_game_sum += game_power;
    }
    println!("Valid game sum: {}", valid_game_sum);
    Ok(())
}