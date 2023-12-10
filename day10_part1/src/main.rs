use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day10_part1_input.txt");
    //let path = Path::new("day10_part1_test2_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new(); 
    let mut start_row = 0;
    let mut start_col = 0;
    for line in reader.lines()
    {
        let line = line?;
        let mut char_vec: Vec<char> = Vec::new();
        for c in line.chars()
        {
            if c == 'S'
            {
                start_row = matrix.len();
                start_col = char_vec.len();
            }   
            char_vec.push(c);
        }
        matrix.push(char_vec);
    }

    //find first two pipes from S
    let connected_pipes = find_connected_pipes(&matrix, (start_row, start_col, '?'));
    let mut current_pos_0 = connected_pipes[0]; 
    let mut current_pos_1 = connected_pipes[1]; 
    let mut steps = 1; 
    while current_pos_0.0 != current_pos_1.0 || current_pos_0.1 != current_pos_1.1
    {
        //find next pipe from current_pos_0
        current_pos_0= one_step(&matrix, current_pos_0);
        //find next pipe from current_pos_1
        current_pos_1 = one_step(&matrix, current_pos_1);
        steps += 1;
    }
    println!("ended on current_pos_0: {:?}", current_pos_0);
    println!("ended on current_pos_1: {:?}", current_pos_1);
    println!("steps: {}", steps);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    Ok(())
}

fn find_connected_pipes(matrix: &Vec<Vec<char>>, current_pos: (usize, usize, char)) -> Vec<(usize, usize, char)>
{
    println!("checking for connected from current_pos: {:?}", current_pos);
    let mut connected_pipes: Vec<(usize, usize, char)> = Vec::new();
    let row = current_pos.0;
    let col = current_pos.1;
    //check north
    if row > 0 && current_pos.2 != 'S' && ['|', 'F', '7'].contains(&matrix[row-1][col]) 
    {
        connected_pipes.push((row-1, col, 'S'));
    }
    //check south
    if row < matrix.len()-1 && current_pos.2 != 'N' && ['|', 'J', 'L'].contains(&matrix[row+1][col])
    {
        connected_pipes.push((row+1, col, 'N'));
    }
    //check east
    if col < matrix[row].len()-1 && current_pos.2 != 'W' && ['-', 'J', '7'].contains(&matrix[row][col+1]) 
    {
        connected_pipes.push((row, col+1, 'W'));
    }
    //check west
    if col > 0 && current_pos.2 != 'E' && ['-', 'F', 'L'].contains(&matrix[row][col-1])
    {
        connected_pipes.push((row, col-1, 'E'));
    }

    println!("connected_pipes: {:?}", connected_pipes);

    if current_pos.2 == '?'
    {
        //assert there is only two connected pipes
        assert_eq!(connected_pipes.len(), 2);
    }
    else
    {
        //assert there is only one connected pipe
        assert_eq!(connected_pipes.len(), 1);
    }
    return connected_pipes;
}

fn one_step(matrix: &Vec<Vec<char>>, current_pos: (usize, usize, char)) -> (usize, usize, char)
{
    let row = current_pos.0;
    let col = current_pos.1;
    let prev = current_pos.2;
    //get pipe type
    let pipe_type = matrix[row][col];
    println!("  checking for connected from current_pos: {:?} with pipe {}", current_pos, pipe_type); 

    if pipe_type == '|'
    {
        if prev == 'N'
        {
            return (row+1, col, 'N');
        }
        else if prev == 'S'
        {
            return (row-1, col, 'S');
        }
        else
        {
            panic!("invalid prev: {}", prev);
        }
    }
    else if pipe_type == '-'
    {
        if prev == 'E'
        {
            return (row, col-1, 'E');
        }
        else if prev == 'W'
        {
            return (row, col+1, 'W');
        }
        else
        {
            panic!("invalid prev: {}", prev);
        }
    }
    else if pipe_type == 'F'
    {
        if prev == 'S'
        {
            return (row, col+1, 'W');
        }
        else if prev == 'E'
        {
            return (row+1, col, 'N');
        }
        else
        {
            panic!("invalid prev: {}", prev);
        }
    }
    else if pipe_type == 'J'
    {
        if prev == 'N'
        {
            return (row, col-1, 'E');
        }
        else if prev == 'W'
        {
            return (row-1, col, 'S');
        }
        else
        {
            panic!("invalid prev: {}", prev);
        }
    }
    else if pipe_type == 'L'
    {
        if prev == 'N'
        {
            return (row, col+1, 'W');
        }
        else if prev == 'E'
        {
            return (row-1, col, 'S');
        }
        else
        {
            panic!("invalid prev: {}", prev);
        }
    }
    else if pipe_type == '7'
    {
        if prev == 'S'
        {
            return (row, col-1, 'E');
        }
        else if prev == 'W'
        {
            return (row+1, col, 'N');
        }
        else
        {
            panic!("invalid prev: {}", prev);
        }
    }
    else
    {
        panic!("invalid pipe_type: {}", pipe_type);
    }
}
