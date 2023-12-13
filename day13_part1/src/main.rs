use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day13_part1_input.txt");
    //let path = Path::new("day13_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut current_puzzle: Vec<Vec<char>> = Vec::new();
    let mut total_answer = 0;
    for line in reader.lines()
    {
        let line = line?;
        if line == ""
        {
            //check reflections
            let tmp = find_reflections(&current_puzzle);
            println!("Answer: {}", tmp);
            total_answer += tmp;
            current_puzzle.clear();
            continue;
        }
        current_puzzle.push(line.chars().collect());
    }
    let tmp = find_reflections(&current_puzzle);
    println!("Answer: {}", tmp);
    total_answer += tmp;

    println!("Total Answer: {}", total_answer);
    Ok(())
}

fn find_reflections(puzzle: &Vec<Vec<char>>) -> u64
{
    //check horizontal
    //find two adjacent rows which are identical
    let mut answer: u64 = 0;
    let mut found_horizontal = false;
    for r1 in 0..puzzle.len()-1
    {
        if puzzle[r1] == puzzle[r1+1]
        {
            println!("Found candidate reflection between rows: {} and {}", r1, r1 + 1);
            if check_horizontal_candidate(puzzle, r1)
            {
                println!("verified horizontal reflection");
                answer = 100 * (r1 + 1) as u64;
                found_horizontal = true;
            }
        }
    }
    if !found_horizontal
    {
        println!("No horizontal reflection found");
    }
    //check vertical
    //find two adjacent columns which are identical
    let mut found_vertical = false;
    //label this loop as outer loop
    for c1 in 0..puzzle[0].len()-1
    {
        //check each row
        let mut is_candidate = true;
        for r1 in 0..puzzle.len()
        {
            if puzzle[r1][c1] != puzzle[r1][c1+1]
            {
                is_candidate = false;
                break; 
            }
        }
        if is_candidate
        {
            println!("Found candidate reflection between columns: {} and {}", c1, c1 + 1);
            if check_vertical_candidate(puzzle, c1)
            {
                println!("verified vertical reflection");
                answer += (c1 + 1) as u64;
                found_vertical = true;
            }
        }
    }
    if !found_vertical
    {
        println!("No vertical reflection found");
    }   
    answer
}

fn check_horizontal_candidate(puzzle: &Vec<Vec<char>>, r: usize) -> bool
{   
    let mut r2 = r + 1;
    let mut r1 = r; 
    while true 
    {
        if puzzle[r1] != puzzle[r2]
        {
            return false;
        }
        if let Some(new_r1) = r1.checked_sub(1)
        {
            r1 = new_r1;
        }
        else
        {
            break;
        }
        r2 += 1;
        if r2 == puzzle.len()
        {
            break;
        }
    }
    true
}

fn check_vertical_candidate(puzzle: &Vec<Vec<char>>, c: usize) -> bool
{
    let mut c2 = c + 1;
    let mut c1 = c;
    while true
    {
        for r in 0..puzzle.len()
        {
            if puzzle[r][c1] != puzzle[r][c2]
            {
                return false;
            }
        }
        if let Some(new_c1) = c1.checked_sub(1)
        {
            c1 = new_c1;
        }
        else
        {
            break;
        }
        c2 += 1;
        if c2 == puzzle[0].len()
        {
            break;
        }
    }
    true
}