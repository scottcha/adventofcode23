use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()>{
    let path = Path::new("day3_part2_input.txt");
    //let path = Path::new("day3_part2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    //go char by char.  if it a . continue.  
    //if its a number check adjacent cells for a number or symbol (non .)
    //if symbol found then add switch bool to part=true
    //if number found then keep adding digit to the number
    
    //read in entire file to a 2d array
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines()
    {
        let line = line?;
        let mut row: Vec<char> = Vec::new();
        for c in line.chars()
        {
            row.push(c);
        }
        grid.push(row);
    }

    //get dimensions of the grid
    let grid_rows = grid.len();
    let grid_cols = grid[0].len();

    let mut part_number_sum: i32 = 0;
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    for row in 0..grid_rows
    {
        //initialize here as a value shouldn't span a row
        let mut on_part = false;
        let mut is_valid_part = false;
        let mut part_number: Option<Vec<char>> = None;
        let mut gear_location: String = String::new();
        for col in 0..grid_cols
        {
            let c = grid[row][col];
            if c.is_numeric()
            {
                //ensure we're evaulating a part
                if on_part
                {
                    part_number.as_mut().unwrap().push(c);
                }
                else
                {
                    //println!("starting parsing part_number {} at row {} col {}", c, row, col);
                    on_part = true;
                    is_valid_part = false;
                    part_number = Some(Vec::new());
                    part_number.as_mut().unwrap().push(c);
                }

                //if we already know this is a valid part, then continue
                if is_valid_part
                {
                    continue;
                }
                //check adjacent cells for a gear symbol '*'
                'outer: for i in -1..2
                {
                    for j in -1..2
                    {
                        //skip the current cell
                        if i == 0 && j == 0
                        {
                            continue;
                        }

                        //check if the adjacent cell is in bounds
                        let adj_row = row as i32 + i;
                        let adj_col = col as i32 + j;
                        if adj_row < 0 || adj_row >= grid_rows as i32 || adj_col < 0 || adj_col >= grid_cols as i32
                        {
                            continue;
                        }

                        //check if the adjacent cell is a symbol
                        let adj_c = grid[adj_row as usize][adj_col as usize];
                        if adj_c.is_numeric() 
                        {
                            continue;
                        }
                        else if adj_c == '.' 
                        {
                            continue;
                        }
                        else if adj_c == '*'
                        {
                            //must be a symbol
                            is_valid_part = true;
                            gear_location = format!("{}, {}", adj_row  , adj_col);
                            //println!("  Determined valid at row {} col {}", row, col);
                            break 'outer;
                        }
                        else
                        {
                            //symbol but not a gear
                            continue;
                        }
                    }
                } 
            }
            else //c == '.'
            {
                if on_part && is_valid_part
                {
                    //convert part_number to a string
                    let tmp_part_number: String = part_number.clone().unwrap().into_iter().collect();
                    println!("found valid part_number: {} ending at row {} col {}", tmp_part_number, row, col);
                    map.entry(gear_location.clone())
                        .or_insert(Vec::new())
                        .push(tmp_part_number.parse::<i32>().unwrap());
                }
                part_number = None;
                on_part = false;
                is_valid_part = false;
                continue;
            }
        } 
        //catch valid part nunbers at the end of the row
        if on_part && is_valid_part
        {
            let tmp_part_number: String = part_number.clone().unwrap().into_iter().collect();
            println!("found valid part_number at line end: {} ending at row {} col {}", tmp_part_number, row, -1);
            map.entry(gear_location.clone())
                .or_insert(Vec::new())
                .push(tmp_part_number.parse::<i32>().unwrap());
        }
    }
    //go through map and every entry with two values, add the product to part_number_sum
    for (_key, value) in map.iter()
    {
        if value.len() == 2
        {
            part_number_sum += value[0] * value[1];
        }
    }
    println!("part_number_sum: {}", part_number_sum);
    Ok(())
}


