use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day7_part1_input.txt");
    //let path = Path::new("day7_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut card_hands: Vec<(String, u32, u32)> = Vec::new();
    for line in reader.lines()
    {
        let line = line?;
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        let cards = line_parts[0];
        let bid = line_parts[1].parse::<u32>().unwrap();
        card_hands.push((cards.to_owned(), bid, card_hand_type(cards)));
    } 

    //sort contents of card_hands first by card_hand_type, then by char value of cards in order of card rank
    card_hands.sort_by(|a, b| 
    {
        if a.2 < b.2
        {
            return Ordering::Less;
        }
        else if a.2 > b.2
        {
            return Ordering::Greater;
        }
        else
        {
            let a_chars: Vec<char> = a.0.chars().collect();
            let b_chars: Vec<char> = b.0.chars().collect();

            for c_i in 0..a_chars.len() {
                if card_order(a_chars[c_i], b_chars[c_i]) == Ordering::Less {
                    return Ordering::Less;
                } else if card_order(a_chars[c_i], b_chars[c_i]) == Ordering::Greater {
                    return Ordering::Greater;
                } else {
                    continue;
                }
            } 
            return Ordering::Equal;
        }
    });
    
    //calculate winnings
    let mut winnings: u32 = 0;
    for i in 0..card_hands.len()
    {
        println!("hand: {}, bid: {}, type: {}", card_hands[i].0, card_hands[i].1, card_hands[i].2);
        winnings += card_hands[i].1 * (i as u32 + 1);
    }
    println!("winnings: {}", winnings);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}

fn card_order(card_a: char, card_b: char) -> Ordering
{
    let card_a_val: u32;
    let card_b_val: u32;
    //special case the chars that are not numbers
    if card_a == 'T'
    {
        card_a_val = 10;
    }
    else if card_a == 'J'
    {
        card_a_val = 11;
    }
    else if card_a == 'Q'
    {
        card_a_val = 12;
    }
    else if card_a == 'K'
    {
        card_a_val = 13;
    }
    else if card_a == 'A'
    {
        card_a_val = 14;
    }
    else
    {
        card_a_val = card_a.to_string().parse::<u32>().unwrap();
    }
    if card_b == 'T'
    {
        card_b_val = 10;
    }
    else if card_b == 'J'
    {
        card_b_val = 11;
    }
    else if card_b == 'Q'
    {
        card_b_val = 12;
    }
    else if card_b == 'K'
    {
        card_b_val = 13;
    }
    else if card_b == 'A'
    {
        card_b_val = 14;
    }
    else
    {
        card_b_val = card_b.to_string().parse::<u32>().unwrap();
    }
    if card_a_val < card_b_val
    {
        return Ordering::Less;
    }
    else if card_a_val > card_b_val
    {
        return Ordering::Greater;
    }
    else
    {
        return Ordering::Equal;
    }
}
//7: five of a kind
//6: four of a kind
//5: full house
//4: three of a kind
//3: two pair
//2: one pair
//1: high card
fn card_hand_type(hand: &str) -> u32
{
    assert!(hand.len() == 5);
    let mut counts: HashMap<char, u32> = HashMap::new();
    for card in hand.chars()
    {
        *counts.entry(card).or_insert(0) += 1;
    }

    //check for 5 of a kind
    if counts.values().any(|&count| count == 5)
    {
        return 7;
    }
    //check for 4 of a kind
    else if counts.values().any(|&count| count == 4)
    {
        return 6;
    }
    //check for full house
    else if counts.values().any(|&count| count == 3) && counts.values().any(|&count| count == 2)
    {
        return 5;
    }
    //check for 3 of a kind
    else if counts.values().any(|&count| count == 3)
    {
        return 4;
    }
    //check for 2 pair
    else if counts.values().filter(|&count| *count == 2).count() == 2
    {
        return 3;
    }
    //check for 1 pair
    else if counts.values().any(|&count| count == 2)
    {
        return 2;
    }
    //high card
    else
    {
        return 1;
    }

}