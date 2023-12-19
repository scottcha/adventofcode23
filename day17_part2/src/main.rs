use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
//help from https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/17.py
fn main() -> io::Result<()>{

    let path = Path::new("day17_part1_input.txt");
    //let path = Path::new("day17_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut distance: HashMap<(usize, usize, i32, i32), u32> = HashMap::new(); //row, col, dir, steps; returns distance
    let mut queue: BinaryHeap<Reverse<(u32, usize, usize, i32, i32)>> = BinaryHeap::new(); //distance, row, col, dir, steps
    for line in reader.lines() 
    {
        let line = line.unwrap();
        let mut row: Vec<u32> = Vec::new();
        let mut sp_row: Vec<bool> = Vec::new();
        for c in line.chars() 
        {
            row.push(c.to_digit(10).unwrap());
        }
        matrix.push(row.clone());
    }

    queue.push(Reverse((0, 0, 0, -1, -1)));

    while !queue.is_empty()
    {
        let Some(Reverse((dist, r, c, dir, steps))) = queue.pop() else {println!("Pop failed"); break;};
        if distance.contains_key(&(r, c, dir, steps)) {
            println!("Already visited ({},{}) with dir {} and indir {}", r, c, dir, steps);
            continue;
        }
        distance.insert((r, c, dir, steps), dist);
        println!("Inserting distnace {} at ({},{}) with dir {} and indir {}", dist, r, c, dir, steps);
        let mut i = 0; //i enumerates the direction
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
        {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            let nd = i;
            let ns = if nd != dir { 1 } else { steps + 1 };

            let isnt_reverse = ((nd + 2) % 4) != dir;

            let is_valid = (ns <= 10 && (nd == dir || steps >= 4 || steps == -1));

            if nr >= 0 && nr < matrix.len() as i32 && nc >= 0 && nc < matrix[0].len() as i32 && is_valid && isnt_reverse
            {
                let cost = matrix[nr as usize][nc as usize];
                if distance.contains_key(&(nr as usize, nc as usize, nd, ns)) {
                    println!("Already visited2 ({},{}) with dir {} and indir {}", r, c, dir, steps);
                    i += 1;
                    continue;
                }
                queue.push(Reverse((dist + cost, nr as usize, nc as usize, nd, ns)));
                println!("Pushing distance {} at ({},{}) with dir {} and indir {}", dist + cost, nr, nc, nd, ns);
            }
            i += 1;
        }
    }

    let mut answer: u32 = std::u32::MAX;    
    for (row, col, dir, steps) in distance.keys() {
        if *row == matrix.len() - 1 && *col == matrix[0].len() - 1 {
            answer = std::cmp::min(answer, distance[&(*row, *col, *dir, *steps)]);
            println!("row: {}, col: {}, dir: {}, steps: {}", row, col, dir, steps);
            println!("Answer is: {}", answer);
        }
    }
    println!("Answer is: {}", answer);
    Ok(())
}

