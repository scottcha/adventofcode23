use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day11_part1_input.txt");
    //let path = Path::new("day11_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new(); 
    let mut galaxy_list: Vec<(i64, i64)> = Vec::new();
    let mut empty_rows: Vec<i64> = Vec::new();
    let mut empty_cols: Vec<i64> = Vec::new();
    for line in reader.lines()
    {
        let line = line?;
        let mut char_vec: Vec<char> = Vec::new();
        let mut galaxy_on_line = false;
        for c in line.chars()
        {
            if c == '#'
            {
                galaxy_on_line = true;
            }   
            char_vec.push(c);
        }
        matrix.push(char_vec);
        if !galaxy_on_line
        {
            //add to lookup if its empty
            empty_rows.push((matrix.len() - 1) as i64);
        }
    }

    //expand the empty cols
    //refactored to be calculated inline to support part2
    // let mut cols_to_add:Vec<i64> = Vec::new();
    // for c in 0..matrix[0].len()
    // {
    //     let mut galaxy_on_col = false;
    //     for r in 0..matrix.len()
    //     {
    //         if matrix[r][c] == '#'
    //         {
    //             galaxy_on_col = true;
    //             break;
    //         }
    //     }
    //     if !galaxy_on_col
    //     {
    //         cols_to_add.push(c as i64);
    //     }
    // }
    // let mut previous_added = 0;
    // for c in cols_to_add
    // {
    //     for r in 0..matrix.len()
    //     {
    //         let insertion_index = c as usize + previous_added;
    //         if insertion_index >= matrix[r].len()
    //         {
    //             matrix[r].push('*');
    //         }
    //         else
    //         {
    //             matrix[r].insert(insertion_index, '*');
    //         }
    //     }
      
    //     previous_added += 1;
    // }

    //create list of empty cols 
    for c in 0..matrix[0].len()
    {
        let mut galaxy_on_col = false;
        for r in 0..matrix.len()
        {
            if matrix[r][c] == '#'
            {
                galaxy_on_col = true;
                break;
            }
        }
        if !galaxy_on_col
        {
            empty_cols.push(c as i64);
        }
    }

    //create the galaxy list
    for r in 0..matrix.len()
    {
        for c in 0..matrix[r].len()
        {
            if matrix[r][c] == '#'
            {
                galaxy_list.push((r as i64, c as i64));
            }
        }
    }
    println!("Galaxy List: {:?}", galaxy_list);
    for r in 0..matrix.len()
    {
        println!("{}: {:?}", r, matrix[r]);
    }

    //find the shortest path between each pair of galaxies.  This seems like its going to be computationally problematic
    //probably easiest to just find the grid offsets
    //only count each pair once
    let mut distance_matrix: Vec<Vec<i64>> = Vec::new();
    let mut total_distance = 0;
    //let size_of_empty = 1; //Part 1: actual size for part 1 is 2 but we just need to count one extra for each
    let size_of_empty = 999999; //Part 2: actual size for part 1 is 2 but we just need to count one extra for each
    println!("Empty Rows: {:?}", empty_rows);
    println!("Empty Cols: {:?}", empty_cols);
    for r in 0..galaxy_list.len()
    {
        let mut row: Vec<i64> = Vec::new();
        for c in r..galaxy_list.len()
        {
            if r == c
            {
                row.push(0);
            }
            else
            {
                //count number of empty rows between the two galaxies
                let mut empty_rows_between = 0;
                for er in 0..empty_rows.len()
                {
                    if empty_rows[er] > galaxy_list[r].0 && empty_rows[er] < galaxy_list[c].0
                    {
                        empty_rows_between += 1;
                    }
                    if empty_rows[er] < galaxy_list[r].0 && empty_rows[er] > galaxy_list[c].0
                    {
                        empty_rows_between += 1;
                    }
                }
                //count number of empty cols between the two galaxies
                let mut empty_cols_between = 0;
                for ec in 0..empty_cols.len()
                {
                    if empty_cols[ec] > galaxy_list[r].1 && empty_cols[ec] < galaxy_list[c].1
                    {
                        empty_cols_between += 1;
                    }
                    if empty_cols[ec] < galaxy_list[r].1 && empty_cols[ec] > galaxy_list[c].1
                    {
                        empty_cols_between += 1;
                    }
                }
                let tmp = (galaxy_list[r].0 - galaxy_list[c].0).abs() + size_of_empty * empty_rows_between  
                          + (galaxy_list[r].1 - galaxy_list[c].1).abs() + size_of_empty * empty_cols_between;
                total_distance += tmp;
                row.push(tmp);
            }
        }
        distance_matrix.push(row);
    }

    println!("Galaxy List: {:?}", galaxy_list);
    for r in 0..distance_matrix.len()
    {
        println!("{}: {:?}", r, distance_matrix[r]);
    }
    println!("Total Distance: {}", total_distance);

    Ok(())
}
