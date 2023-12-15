use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()>{


    let path = Path::new("day14_part1_input.txt");
    //let path = Path::new("day14_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut load: u64 = 0;
    for line in reader.lines()
    {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    let start = Instant::now();
    //move each O up until it either hits the well, another O or a #
    for c in 0..matrix[0].len()
    {
        for r in 0..matrix.len()
        {
            if matrix[r][c] == 'O'
            {
                let mut tmp_r = r;
                while tmp_r > 0
                {
                    if matrix[tmp_r-1][c] == '#'
                    {
                        break;
                    }
                    if matrix[tmp_r-1][c] == 'O'
                    {
                        break;
                    }
                    matrix[tmp_r][c] = '.';
                    matrix[tmp_r-1][c] = 'O';
                    tmp_r -= 1;
                }
                load += matrix.len() as u64 - tmp_r as u64;
            }
        }
    }
    for r in 0..matrix.len()
    {
        println!("{:?}", matrix[r]);
    }
    println!("Load: {}", load);
    
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}