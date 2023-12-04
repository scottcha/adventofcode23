use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() -> io::Result<()>{
    let path = Path::new("day4_part2_input.txt");
    //let path = Path::new("day3_part2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    const TOTAL_GAMES: usize = 213; //hardcoded for now to avoid having to parse the file twice
    let mut card_copies = vec![1; TOTAL_GAMES];
    card_copies.fill(1); //initialize to all 1s
    let mut total_score: i32 = 0;
    let mut current_card = 0;
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

        let mut current_matching_cards = 0;
        for number in my_numbers
        {
            let inumber = number.parse::<i32>().unwrap();
            if set.contains(&inumber)
            {
                current_matching_cards += 1;
            }
        }
        println!("current_matching_cards: {}", current_matching_cards);
        //update the card_copies array
        for _j in 0..card_copies[current_card]
        {
            for i in current_card + 1..current_card + 1 + current_matching_cards
            {
                card_copies[i] += 1;
            }
        }
        current_card += 1;

    }
    //count the values of card_copies
    for i in &card_copies
    {
        total_score += i; 
    }
    println!("total_score: {}", total_score);
    Ok(())
}