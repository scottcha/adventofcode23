use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
fn main() -> io::Result<()>{

    //let path = Path::new("day17_part1_input.txt");
    let path = Path::new("day17_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut sp_matrix: Vec<Vec<bool>> = Vec::new();
    let mut distance: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();
        let mut sp_row: Vec<bool> = Vec::new();
        let mut distance_row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c);
            distance_row.push(std::u32::MAX);
            sp_row.push(false);
        }
        matrix.push(row.clone());
        sp_matrix.push(sp_row.clone());
        distance.push(distance_row.clone());
    }

    distance[0][0] = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut min_index = min_distance(distance, sp_matrix);
        }
    }   

    sp_matrix[min_index.0][min_index.1] = true;
    

    Ok(())
}

fn min_distance(distance: &Vec<Vec<u32>>, sp_matrix: &Vec<Vec<bool>>) -> (u32, u32) {
    let mut min = std::u32::MAX;
    let mut min_index = (-1, -1);
    for i in 0..distance.len() {
        for j in 0..distance[i].len() {
            if !sp_matrix[i][j] && distance[i][j] <= min {
                min = distance[i][j];
                min_index = (i as u32, j as u32);
            }
        }
    }
    min_index
}