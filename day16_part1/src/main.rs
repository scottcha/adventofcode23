use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
//bug in this solution when running for part two.  The solution found is 13 higher than actual solution.  
fn main() -> io::Result<()>{

    let path = Path::new("day16_part1_input.txt");
    //let path = Path::new("day16_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut traveled_matrix: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();
        let mut traveled_row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c);
            traveled_row.push(1);
        }
        matrix.push(row.clone());
        traveled_matrix.push(traveled_row.clone());
    }
    print_matrix(&matrix);
    //start at top left
    //move_beam(&matrix, &mut traveled_matrix, 0, 0, 'r');
    //print_traveled_matrix(&traveled_matrix);

    //part 2
    let mut best = 0;
    
    for i in 0..matrix[0].len()
    {
        clear_traveled(&mut traveled_matrix);
        move_beam(&matrix, &mut traveled_matrix, 0, i, 'd');
        let energized = count_energized(&traveled_matrix);
        if energized > best {
            best = energized;
            println!("New best (d): at {}, X: 0, Y: {}", energized, i);
        }
        
        clear_traveled(&mut traveled_matrix);
        move_beam(&matrix, &mut traveled_matrix, matrix[0].len()-1, i, 'u');
        let energized = count_energized(&traveled_matrix);
        if energized > best {
            best = energized;
            println!("New best (u): at {}, X: 0, Y: {}", energized, i);
        }
    }
    for i in 0..matrix.len() {
        clear_traveled(&mut traveled_matrix);
        move_beam(&matrix, &mut traveled_matrix, i, 0, 'r');
        let energized = count_energized(&traveled_matrix);
        if energized > best {
            best = energized;
            println!("New best (r): at {}, X: {}, Y: 0", energized, i);
        }
        
        clear_traveled(&mut traveled_matrix);
        move_beam(&matrix, &mut traveled_matrix, i, matrix.len()-1, 'l');
        let energized = count_energized(&traveled_matrix);
        if energized > best {
            best = energized;
            println!("New best (l): at {}, X: {}, Y: 0", energized, i);
        }
    }
    clear_traveled(&mut traveled_matrix);
    move_beam(&matrix, &mut traveled_matrix, 27, 0, 'r');    
    print_traveled_matrix(&traveled_matrix);
    Ok(())
}

fn clear_traveled(traveled_matrix: &mut Vec<Vec<u32>>) {
    for i in 0..traveled_matrix.len() {
        for j in 0..traveled_matrix[0].len() {
            traveled_matrix[i][j] = 1;
        }
    }
}

fn check_traveled(traveled_value: u32, direction: char) -> bool
{
    //println!("Checking traveled value {} with direction {}", traveled_value, direction);
    match direction
    {
        'r' => {
            if traveled_value % 2 == 0 {
                return true;
            }
            else {
                return false;
            }
        }
        'd' => {
            if traveled_value % 3 == 0 {
                return true;
            }
            else {
                return false;
            }
        }
        'l' => {
            if traveled_value % 5 == 0 {
                return true;
            }
            else {
                return false;
            }
        }
        'u' => {
            if traveled_value % 7 == 0 {
                return true;
            }
            else {
                return false;
            }
        }
        _ => {
            //println!("ERROR: invalid direction");
            return false;
        }
    }
}

fn update_traveled(traveled_matrix: &mut Vec<Vec<u32>>, x: usize, y: usize, direction: char)
{
    //println!("Updating traveled at ({}, {}) with direction {}", x, y, direction);
    match direction
    {
        'r' => {
            traveled_matrix[x][y] *= 2;
        }
        'd' => {
            traveled_matrix[x][y] *= 3;
        }
        'l' => {
            traveled_matrix[x][y] *= 5;
        }
        'u' => {
            traveled_matrix[x][y] *= 7;
        }
        _ => {
            //println!("ERROR: invalid direction");
        }
    }
}
fn move_beam(matrix: &Vec<Vec<char>>, traveled_matrix: &mut Vec<Vec<u32>>, mut x: usize, mut y: usize, mut direction: char) 
{
    //let travel_threshold = 4;
    loop 
    {
        //advance in direction until hit splitter or mirror
        while x != matrix.len() && y != matrix[0].len() && matrix[x][y] == '.' && !check_traveled(traveled_matrix[x][y], direction) {
            update_traveled(traveled_matrix, x, y, direction);
            match advance(x, y, direction) {
                Ok((newx, newy)) => {
                    x = newx;
                    y = newy;
                }
                Err(_) => {
                    //println!("ERROR: in . advance");
                }
            }
            //println!("  X: {}, Y: {}", x, y);
        }
        
        //base cases
        if x == matrix.len() || y == matrix[0].len() {
            return; 
        }

        //println!("X: {}, Y: {}", x, y);
        //already traveled here on this beam
        if check_traveled(traveled_matrix[x][y], direction) { 
            //println!("Already traveled here X: {}, Y: {}", x, y);
            return;
        }
        update_traveled(traveled_matrix, x, y, direction);
        //hit mirror
        if matrix[x][y] == '/'
        {
            if direction == 'r' {
                direction = 'u';
            }
            else if direction == 'd' {
                direction = 'l';
            }
            else if direction == 'l' {
                direction = 'd';
            }
            else if direction == 'u' {
                direction = 'r';
            }
            else {
                //println!("ERROR: invalid direction");
            }
            match advance(x, y, direction) {
                Ok((newx, newy)) => {
                    x = newx;
                    y = newy;
                }
                Err(_) => {
                    //println!("ERROR: in / advance");
                }
            }
        }
        else if matrix[x][y] == '\\'
        {
            //println!("Hit \\ with direction {} and x {} y {}", direction, x, y);
            if direction == 'r' {
                direction = 'd';
            }
            else if direction == 'd' {
                direction = 'r';
            }
            else if direction == 'l' {
                direction = 'u';
            }
            else if direction == 'u' {
                direction = 'l';    
            }
            else {
                //println!("ERROR: invalid direction");
            }
            match advance(x, y, direction) {
                Ok((newx, newy)) => {
                    x = newx;
                    y = newy;
                }
                Err(_) => {
                    //println!("ERROR: in \\ advance");
                }
            }
        }
        else if matrix[x][y] == '|'
        {
            //println!("Split |");
            if direction == 'l' || direction == 'r' {
                //println!("Splitting up and down");
                //let (newx, newy) = advance(x, y, 'u')?;
                match advance(x, y, 'u') {
                    Ok((newx, newy)) => {
                        move_beam(matrix, traveled_matrix, newx, newy, 'u');                    
                    }
                    Err(_) => 
                    {
                        //println!("ERROR: in u");    
                    }
                }
                match advance(x, y, 'd') {
                    Ok((newx, newy)) => {
                        move_beam(matrix, traveled_matrix, newx, newy, 'd');                     
                    }
                    Err(_) => 
                    {
                        //println!("ERROR: in d");    
                    }
                }
                //let (newx, newy) = advance(x, y, 'd')?;
                return; 
            }
            else if direction == 'u' {
                match advance(x, y, direction) {
                    Ok((newx, newy)) => {
                        x = newx;
                        y = newy;
                    }
                    Err(_) => {
                        //println!("ERROR: in u advance");
                    }
                }
            }
            else if direction == 'd' {
                match advance(x, y, direction) {
                    Ok((newx, newy)) => {
                        x = newx;
                        y = newy;
                    }
                    Err(_) => {
                        //println!("ERROR: in d advance");
                    }
                }
            }
            else {
                //println!("ERROR: invalid direction");
            }
        }
        else if matrix[x][y] == '-'
        {
            if direction == 'u' || direction == 'd' {
               
                match advance(x, y, 'l') {
                    Ok((newx, newy)) => {
                        move_beam(matrix, traveled_matrix, newx, newy, 'l');
                    }
                    Err(_) => 
                    {
                        //println!("ERROR: in l");
                    }
                }
                match advance(x, y, 'r') {
                    Ok((newx, newy)) => {
                        move_beam(matrix, traveled_matrix, newx, newy, 'r');
                    }
                    Err(_) => 
                    {
                        //println!("ERROR: in r");
                    }
                }
                return;
            }
            else if direction == 'l' {
                match advance(x, y, direction) {
                    Ok((newx, newy)) => {
                        x = newx;
                        y = newy;
                    }
                    Err(_) => {
                        //println!("ERROR: in l advance");
                    }
                }
            }
            else if direction == 'r' {
                match advance(x, y, direction) {
                    Ok((newx, newy)) => {
                        x = newx;
                        y = newy;
                    }
                    Err(_) => {
                        //println!("ERROR: in r advance");
                    }
                }
            }
            else {
                println!("ERROR: invalid direction");
            }
        }
        else
        {
            println!("ERROR: invalid character {} at ({}, {})", matrix[x][y], x, y);
        }
    }
}


fn advance(x: usize, y: usize, direction: char) -> Result<(usize, usize), String>
{
    let max_x = std::usize::MAX;
    let max_y = std::usize::MAX;

    if direction == 'r' && y < max_y {
        Ok((x, y + 1))
    }
    else if direction == 'd' && x < max_x {
        Ok((x + 1, y))
    }
    else if direction == 'l' && y > 0 {
        Ok((x, y - 1))
    }
    else if direction == 'u' && x > 0 {
        Ok((x - 1, y))
    }
    else {
        //format error string
        let error_string = format!("ERROR: out of bounds x {} y {} direction {}", x, y, direction);
        Err(error_string)
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{}", matrix[i][j]);
        }
        println!("");
    }
}
fn print_traveled_matrix(matrix: &Vec<Vec<u32>>) {
    let mut energized = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] > 1 {
                energized += 1;
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
    println!("Energized: {}", energized);
}

fn count_energized(matrix : &Vec<Vec<u32>>) -> u32
{
    let mut energized = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] > 1 {
                energized += 1;
            }
        }
    }
    return energized;
}