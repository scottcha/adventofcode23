use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day10_part2_input.txt");
    //let path = Path::new("day10_part2_test3_input.txt");
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

    //create a distance matrix with same dims as matrix
    let mut distance_matrix: Vec<Vec<char>> = Vec::new();
    for row in 0..matrix.len()
    {
        let mut char_vec: Vec<char> = Vec::new();
        for col in 0..matrix[row].len()
        {
            char_vec.push(' ');
        }
        distance_matrix.push(char_vec);
    }
    //find first two pipes from S
    let connected_pipes = find_connected_pipes(&matrix, (start_row, start_col, '?'));
    distance_matrix[start_row][start_col] = 'P';
    let mut current_pos_0 = connected_pipes[0]; 
    let mut current_pos_1 = connected_pipes[1]; 
    let mut steps = 1; 
    while current_pos_0.0 != current_pos_1.0 || current_pos_0.1 != current_pos_1.1
    {
        if ['L', '7'].contains(&matrix[current_pos_0.0][current_pos_0.1] )
        { 
            distance_matrix[current_pos_0.0][current_pos_0.1] = 'Z';
        }
        else
        {
            distance_matrix[current_pos_0.0][current_pos_0.1] = 'P';
        }
        
        if ['L', '7'].contains(&matrix[current_pos_1.0][current_pos_1.1] )
        { 
            distance_matrix[current_pos_1.0][current_pos_1.1] = 'Z';
        }
        else
        {
            distance_matrix[current_pos_1.0][current_pos_1.1] = 'P';
        }
        //find next pipe from current_pos_0
        current_pos_0= one_step(&matrix, current_pos_0);
        //find next pipe from current_pos_1
        current_pos_1 = one_step(&matrix, current_pos_1);
        //mark the positions as pipes
        steps += 1;
    }
    distance_matrix[current_pos_0.0][current_pos_0.1] = 'P';
    println!("steps: {}", steps);
    // for row in 0..distance_matrix.len()
    // {
    //     println!("distance_matrix[{}]: {:?}", row, distance_matrix[row]);
    // }
    let mut col_inside_count = 0;
    for row in 0..distance_matrix.len()
    {
        for col in 0..distance_matrix[row].len()
        {
            if ['P', 'Z'].contains(&distance_matrix[row][col])
            {
                continue;
            }
            let res = count_pipes_from_pos(&distance_matrix, (row, col));
            //debug
            // if row == 6 && col == 2
            // {
            //     println!("row 6 col 2 res: {:?}", res);
            // }
            if res.1 == 'I'
            {
                col_inside_count += 1;
            }
            distance_matrix[row][col] = res.1;
        }
    }

    println!("col_inside_count: {}", col_inside_count);
    // for row in 0..distance_matrix.len()
    // {
    //     println!("distance_matrix[{}]: {:?}", row, distance_matrix[row]);
    // }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    Ok(())
}

fn count_pipes_from_pos(matrix: &Vec<Vec<char>>, current_pos: (usize, usize)) -> (i32, char)
{
    let mut pipe_count = 0;
    let mut i = current_pos.0+1;
    let mut j = current_pos.1+1;
    //count SE
    while true
    { 
        if i >= matrix.len() || j >= matrix[i].len()
        {
            break;
        }
        if matrix[i][j] == 'P'
        {
            pipe_count += 1;
        }
        i = i + 1;
        j = j + 1;
    }
    
    if pipe_count % 2 == 1
    {
        return (pipe_count, 'I');
    }
    else
    {
        return (pipe_count, 'O');
    }
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

