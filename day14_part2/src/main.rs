use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use std::collections::{HashSet, HashMap};

fn main() -> io::Result<()>{

    let path = Path::new("day14_part2_input.txt");
    //let path = Path::new("day14_part2_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut O_matrix: HashSet<(u32, u32)> = HashSet::new();
    let mut B_matrix: HashSet<(u32, u32)> = HashSet::new();
    let mut load: u64 = 0;
    let mut row_size: u32 = 0;
    let mut col_size: u32 = 0;
    // let mut n_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = Some(HashMap::new());  
    // let mut s_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = Some(HashMap::new());  
    // let mut e_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = Some(HashMap::new());  
    // let mut w_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = Some(HashMap::new());  
    let mut n_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = None;
    let mut s_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = None;
    let mut e_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = None;
    let mut w_cache: Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>> = None;

    for line in reader.lines()
    {
        let line = line?;
        for c in 0..line.len()
        {
            let tmp_c = line.chars().nth(c).unwrap();
            if tmp_c == 'O'
            {
                //println!("Found O at x: {}, y: {}", row_size, c);
                O_matrix.insert((row_size as u32, c as u32));
            }
            else if tmp_c == '#'
            {
                B_matrix.insert((row_size as u32, c as u32));
            }
            else 
            {
                continue;
            }
        }
        row_size += 1;
        if col_size == 0
        {
            col_size = line.len() as u32;
        }   
    }

    let start = Instant::now();
    //10,000 took 1.89 seconds with no cache on test input
    //100 iterations to 8 seconds with cache
    //100 iterations to 3.7 seconds without cache
    //1000 iterations to 36 seconds without cache
    //1000 iterations to 73 seconds with cache
    let n_iterations = 1000;//1000000000;
    for iterations in 0..n_iterations
    {
        if iterations % 1 == 0
        {
            print!("Iteration: {} ", iterations);
            //estimated time remaining
            let duration = start.elapsed();
            let time_remaining = (duration.as_secs() as f64 / iterations as f64) * (n_iterations - iterations) as f64;
            //println!("Estimated time remaining: {} minutes", time_remaining/60.0);
            print_matrix(&mut O_matrix, &mut B_matrix, row_size, col_size, true);
        }
        //move each O up until it either hits the wall, another O or a #
        //print_matrix(&mut O_matrix, &mut B_matrix, row_size, col_size);
        //println!("");
        //println!("O_matrix keys: {:?}", O_matrix.iter());
        move_n(&mut O_matrix, &mut B_matrix, row_size, col_size, &mut n_cache); 
        //print_matrix(&mut O_matrix, &mut B_matrix, row_size, col_size, false);
        move_w(&mut O_matrix, &mut B_matrix, row_size, col_size, &mut w_cache); 
        //print_matrix(&mut O_matrix, &mut B_matrix, row_size, col_size, false);
        move_s(&mut O_matrix, &mut B_matrix, row_size, col_size, &mut s_cache); 
        //print_matrix(&mut O_matrix, &mut B_matrix, row_size, col_size, false);
        move_e(&mut O_matrix, &mut B_matrix, row_size, col_size, &mut e_cache); 
        //print_matrix(&mut O_matrix, &mut B_matrix, row_size, col_size, false);
    }
    move_n(&mut O_matrix, &mut B_matrix, row_size, col_size, &mut n_cache); 

    print_matrix(&mut O_matrix, &mut B_matrix, row_size, col_size, false);
    
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}

fn print_matrix(O_matrix: &mut HashSet<(u32, u32)>, B_matrix: &mut HashSet<(u32, u32)>, row_size: u32, col_size: u32, only_load: bool)
{
    let mut O_count = 0;
    let mut load = 0;
    for x in 0..row_size
    {
        for y in 0..col_size
        {
            if O_matrix.contains(&(x, y))
            {
                O_count += 1;
                load += row_size as u64 - x as u64;
                if !only_load
                {
                    print!("O");
                } 
            }
            else if B_matrix.contains(&(x, y))
            {
                if !only_load
                {
                    print!("#");
                }
            }
            else
            {
                if !only_load
                {
                    print!(".");
                }
            }
        }
        if !only_load
        {
            println!("");
        }
    }
    if !only_load
    {
        println!("");
        println!("O_count: {}", O_count);
    }
    println!("load: {}", load);
    if !only_load
    {
        println!("");
    }
}

fn move_n(O_matrix: &mut HashSet<(u32, u32)>, B_matrix: &mut HashSet<(u32, u32)>, row_size: u32, col_size: u32, n_cache: &mut Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>>)
{
    let mut O_keys: Vec<(u32, u32)> = O_matrix.iter().cloned().collect();
    //println!("O_keys: {:?}", O_keys);
    O_keys.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    //clone O_keys

    let mut O_keys_orig: Vec<(u32, u32)> = O_keys.clone();
    if let Some(cache) = n_cache 
    {
        for c in 0..col_size
        {
            //filter O_keys by c
            let mut O_keys_c: Vec<(u32, u32)> = O_keys.iter().filter(|&x| x.1 == c).cloned().collect();
            if let Some(O_keys_new) = cache.get(&(c, O_keys_c.clone())) {
                //println!("N Cache hit keylen before : {}", O_keys.len());
                //remove O_keys_c from O_keys and O_matrix and insert O_keys_new into O_matrix
                for &(x, y) in O_keys_c.iter()
                {
                    O_matrix.remove(&(x, y));
                    //remove x, y from O_keys
                    let index = O_keys.iter().position(|&r| r == (x, y)).unwrap();
                    O_keys.remove(index);
                }
                for &(x, y) in O_keys_new.iter()
                {
                    O_matrix.insert((x, y));
                }
                //println!("  hit keylen after: {}", O_keys.len());
            }
        }
    }
    //println!("N Searching for keys with len: {}", O_keys.len());
    for &(x, y) in O_keys.iter()
    {
        //println!("On key x: {}, y: {}", x, y);
        if x == 0
        {
            //println!("x == 0");
            continue;
        }
        else if B_matrix.contains(&(x-1, y))
        {
            //println!("B_matrix contains x: {}, y: {}", x-1, y);
            continue;
        }
        else if O_matrix.contains(&(x-1, y)) 
        {
            //println!("O_matrix contains x: {}, y: {}", x-1, y);
            continue;
        }
        else
        {
            //print!("N");
            let mut tmp_x: u32 = x-1;
            while tmp_x > 0 && !B_matrix.contains(&(tmp_x, y)) && !O_matrix.contains(&(tmp_x, y))
            {
                tmp_x -= 1;
            }
            if B_matrix.contains(&(tmp_x, y)) || O_matrix.contains(&(tmp_x, y))
            {
                tmp_x += 1;
            }
            //println!("Moving x: {}, y: {}, tmp_x: {}", x, y, tmp_x);
            //remove from O_matrix
            O_matrix.remove(&(x, y));
            //insert into O_matrix
            O_matrix.insert((tmp_x, y));
        }
    }
    //insert in to cache
    if let Some(cache) = n_cache 
    {
        for c in 0..col_size
        {
            //filter O_Matrix by c
            let mut O_keys_c: Vec<(u32, u32)> = O_matrix.iter().filter(|&x| x.1 == c).cloned().collect();
            O_keys_c.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
            //filter O_keys_orig by c
            let O_keys_orig_c: Vec<(u32, u32)> = O_keys_orig.iter().filter(|&x| x.1 == c).cloned().collect();
            //println!("Inserting into cache: {:?} -> {:?}", (c, O_keys_orig_c.clone()), O_keys_c.clone());
            cache.insert((c, O_keys_orig_c), O_keys_c);
        }
    }
}

fn move_e(O_matrix: &mut HashSet<(u32, u32)>, B_matrix: &mut HashSet<(u32, u32)>, row_size: u32, col_size: u32, e_cache: &mut Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>>)
{
    let mut O_keys: Vec<(u32, u32)> = O_matrix.iter().cloned().collect();
    //sort by lowest row then highest col
    O_keys.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let mut O_keys_orig: Vec<(u32, u32)> = O_keys.clone();
    if let Some(cache) = e_cache 
    {
        for r in 0..row_size
        {
            //filter O_keys by r
            let mut O_keys_c: Vec<(u32, u32)> = O_keys.iter().filter(|&x| x.0 == r).cloned().collect();
            if let Some(O_keys_new) = cache.get(&(r, O_keys_c.clone())) {
                //println!("E Cache hit");
                //remove O_keys_c from O_keys and O_matrix and insert O_keys_new into O_matrix
                for &(x, y) in O_keys_c.iter()
                {
                    O_matrix.remove(&(x, y));
                    //remove x, y from O_keys
                    let index = O_keys.iter().position(|&i| i == (x, y)).unwrap();
                    O_keys.remove(index);
                }
                for &(x, y) in O_keys_new.iter()
                {
                    O_matrix.insert((x, y));
                }
            }
        }
    }

    //println!("E Searching for keys with len: {}", O_keys.len());
    for &(x, y) in O_keys.iter()
    {
        if y == col_size-1 
        {
            continue;
        }
        else if B_matrix.contains(&(x, y+1))
        {
            continue;
        }
        else if O_matrix.contains(&(x, y+1)) 
        {
            continue;
        }
        else
        {
            //print!("E");
            let mut tmp: u32 = y+1;
            while tmp < col_size-1 && !B_matrix.contains(&(x, tmp)) && !O_matrix.contains(&(x, tmp))
            {
                tmp += 1;
            }
            if B_matrix.contains(&(x, tmp)) || O_matrix.contains(&(x, tmp))
            {
                tmp -= 1;
            }
            //remove from O_matrix
            O_matrix.remove(&(x, y));
            //insert into O_matrix
            O_matrix.insert((x, tmp));
        }
    }
    
    //insert in to cache
    if let Some(cache) = e_cache 
    {
        for r in 0..row_size
        {
            //filter O_Matrix by c
            let mut O_keys_c: Vec<(u32, u32)> = O_matrix.iter().filter(|&x| x.0 == r).cloned().collect();
            O_keys_c.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
            //filter O_keys_orig by c
            let O_keys_orig_c: Vec<(u32, u32)> = O_keys_orig.iter().filter(|&x| x.0 == r).cloned().collect();
            //println!("Inserting into cache: {:?} -> {:?}", (r, O_keys_orig_c.clone()), O_keys_c.clone());
            cache.insert((r, O_keys_orig_c), O_keys_c);
        }
    }
}

fn move_s(O_matrix: &mut HashSet<(u32, u32)>, B_matrix: &mut HashSet<(u32, u32)>, row_size: u32, col_size: u32, s_cache: &mut Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>>)
{
    let mut O_keys: Vec<(u32, u32)> = O_matrix.iter().cloned().collect();
    //sort by higest row then lowest col
    O_keys.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut O_keys_orig: Vec<(u32, u32)> = O_keys.clone();
    if let Some(cache) = s_cache 
    {
        for c in 0..col_size
        {
            //filter O_keys by c
            let mut O_keys_c: Vec<(u32, u32)> = O_keys.iter().filter(|&x| x.1 == c).cloned().collect();
            if let Some(O_keys_new) = cache.get(&(c, O_keys_c.clone())) {
                //println!("S Cache hit");
                //remove O_keys_c from O_keys and O_matrix and insert O_keys_new into O_matrix
                for &(x, y) in O_keys_c.iter()
                {
                    O_matrix.remove(&(x, y));
                    //remove x, y from O_keys
                    let index = O_keys.iter().position(|&r| r == (x, y)).unwrap();
                    O_keys.remove(index);
                }
                for &(x, y) in O_keys_new.iter()
                {
                    O_matrix.insert((x, y));
                }
            }
        }
    }
    //println!("S Searching for keys with len: {}", O_keys.len());
    for &(x, y) in O_keys.iter()
    {
        if x == row_size-1 
        {
            continue;
        }
        else if B_matrix.contains(&(x+1, y))
        {
            continue;
        }
        else if O_matrix.contains(&(x+1, y)) 
        {
            continue;
        }
        else
        {
            //print!("S");
            let mut tmp: u32 = x+1;
            while tmp < row_size-1 && !B_matrix.contains(&(tmp, y)) && !O_matrix.contains(&(tmp, y))
            {
                tmp += 1;
            }
            if B_matrix.contains(&(tmp, y)) || O_matrix.contains(&(tmp, y))
            {
                tmp -= 1;
            }
            //remove from O_matrix
            O_matrix.remove(&(x, y));
            //insert into O_matrix
            O_matrix.insert((tmp, y));
        }
    }  
    //insert in to cache
    if let Some(cache) = s_cache 
    {
        for c in 0..col_size
        {
            //filter O_Matrix by c
            let mut O_keys_c: Vec<(u32, u32)> = O_matrix.iter().filter(|&x| x.1 == c).cloned().collect();
            O_keys_c.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            //filter O_keys_orig by c
            let O_keys_orig_c: Vec<(u32, u32)> = O_keys_orig.iter().filter(|&x| x.1 == c).cloned().collect();
            //println!("Inserting into cache: {:?} -> {:?}", (c, O_keys_orig_c.clone()), O_keys_c.clone());
            cache.insert((c, O_keys_orig_c), O_keys_c);
        }
    }
}

fn move_w(O_matrix: &mut HashSet<(u32, u32)>, B_matrix: &mut HashSet<(u32, u32)>, row_size: u32, col_size: u32, w_cache: &mut Option<HashMap<(u32, Vec<(u32, u32)>), Vec<(u32, u32)>>>)
{
    let mut O_keys: Vec<(u32, u32)> = O_matrix.iter().cloned().collect();
    //sort by lowest row then hightest col
    O_keys.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut O_keys_orig: Vec<(u32, u32)> = O_keys.clone();
    if let Some(cache) = w_cache 
    {
        for r in 0..row_size
        {
            //filter O_keys by r
            let mut O_keys_c: Vec<(u32, u32)> = O_keys.iter().filter(|&x| x.0 == r).cloned().collect();
            if let Some(O_keys_new) = cache.get(&(r, O_keys_c.clone())) {
                //println!("W Cache hit");
                //remove O_keys_c from O_keys and O_matrix and insert O_keys_new into O_matrix
                for &(x, y) in O_keys_c.iter()
                {
                    O_matrix.remove(&(x, y));
                    //remove x, y from O_keys
                    let index = O_keys.iter().position(|&i| i == (x, y)).unwrap();
                    O_keys.remove(index);
                }
                for &(x, y) in O_keys_new.iter()
                {
                    O_matrix.insert((x, y));
                }
            }
        }
    }
    //println!("W Searching for keys with len: {}", O_keys.len());
    for &(x, y) in O_keys.iter()
    {
        if y == 0 
        {
            continue;
        }
        else if B_matrix.contains(&(x, y-1))
        {
            continue;
        }
        else if O_matrix.contains(&(x, y-1)) 
        {
            continue;
        }
        else
        {
            //print!("W");
            let mut tmp: u32 = y-1;
            while tmp > 0 && !B_matrix.contains(&(x, tmp)) && !O_matrix.contains(&(x, tmp))
            {
                tmp -= 1;
            }
            if B_matrix.contains(&(x, tmp)) || O_matrix.contains(&(x, tmp))
            {
                tmp += 1;
            }
            //remove from O_matrix
            O_matrix.remove(&(x, y));
            //insert into O_matrix
            O_matrix.insert((x, tmp));
        }
    }
    //insert in to cache
    if let Some(cache) = w_cache 
    {
        for r in 0..row_size
        {
            //filter O_Matrix by c
            let mut O_keys_c: Vec<(u32, u32)> = O_matrix.iter().filter(|&x| x.0 == r).cloned().collect();
            O_keys_c.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
            //filter O_keys_orig by c
            let O_keys_orig_c: Vec<(u32, u32)> = O_keys_orig.iter().filter(|&x| x.0 == r).cloned().collect();
            //println!("Inserting into cache: {:?} -> {:?}", (r, O_keys_orig_c.clone()), O_keys_c.clone());
            cache.insert((r, O_keys_orig_c), O_keys_c);
        }
    }
}