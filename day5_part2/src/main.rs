use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

// pre optimization
// lowest seed id: 2254686
// highest seed id: 4102374000
// Time elapsed is: 100.8056041s
// 4 threads debug Time elapsed is: 380.5795926s
// 12 threads debug Time elapsed is: 193.5838783s
// 20 threads debug Time elapsed is: 170.1574675s
// 24 threads debug Time elapsed is: 166.9757496s
// 24 threads release Time elapsed is: 28.5595146s
fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day5_part2_input.txt");
    //let path = Path::new("day5_part2_test_input.txt");
    let file = File::open(&path)?;
    let mut reader = io::BufReader::new(file);

    //read first line which is the seed ids
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut seed_ids: Vec<u64> = Vec::new();
    
    {
        //let the immutable borrow of line go out of scope
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        //skip the seeds: which is index 0
        let mut on_seed = true;
        let mut seed_id = 0;
        for part in line_parts[1..].iter()
        {
            println!("part: {}", part);
            if on_seed
            {
                seed_id = part.parse::<u64>().unwrap();
                seed_ids.push(seed_id);
                on_seed = false;
                continue;
            }
            else
            {
                assert!(seed_id != 0);
                for i in seed_id..seed_id + part.parse::<u64>().unwrap()
                {
                    seed_ids.push(i);
                }
                on_seed = true;
                seed_id = 0;
            }
        }
    }

    let mut mapping: Vec<(u64, u64, u64)> = Vec::new();

    //read blank and then read through next blank line & the map name
    reader.read_line(&mut line)?;
    for line in reader.lines()
    {
        let line = line?;
        //print line with hidden chars visible
        //println!("line: {:?}", line);
        //check for next map
        if line == ""
        {
            //println!("Found newline");
            //use the current map to do the mapping
            assert!(mapping.len() > 0);
            update_mapping(&mut mapping, &mut seed_ids);
        }
        else if line.contains("map")
        {
            //println!("Found map string");
            //read the map name
            continue;
        }
        else if line.contains("EOF")
        {
            break;
        }
        else
        {
            println!("creating map");
            println!("line: {}", line);
            //read the map
            let line_parts: Vec<&str> = line.split_whitespace().collect();
            mapping.push((line_parts[1].parse::<u64>().unwrap(), line_parts[0].parse::<u64>().unwrap(), line_parts[2].parse::<u64>().unwrap()));
        }
    }

    //sort seed_ids and get the lowest one
    seed_ids.sort();
    println!("lowest seed id: {}", seed_ids[0]);
    //print last seed_id
    println!("highest seed id: {}", seed_ids[seed_ids.len() - 1]);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}

fn update_mapping(mapping: &mut Vec<(u64, u64, u64)>, seed_ids: &mut Vec<u64>) {
    //println!("In mapping with mapping len: {}", mapping.len()); 
    //sort the mapping by the first element
    mapping.sort_by(|a, b| a.0.cmp(&b.0));
    //for each seed id, do the mapping
    //for seed_id in seed_ids
    let pool = ThreadPoolBuilder::new().num_threads(24).build().unwrap();
    pool.install(|| {
        seed_ids.par_iter_mut().for_each(|seed_id|
        {
            //go through the mapping and find the appropriate range if one exist
            for map in &*mapping
            {   
                //println!(" checking seed_id: {} with map: {:?}", *seed_id, map);
                if *seed_id >= map.0 && *seed_id < map.0 + map.2
                {
                    //println!("found range");
                    let offset = *seed_id - map.0;
                    let new_id = map.1 + offset;
                    //println!("mapping seed_id: {} to new_id: {}", *seed_id, new_id);
                    *seed_id = new_id;
                    break;
                }
            }
            //since default mapping is 1:1, we don't need to do anything if no range is found 
        });
    });
    //clear the map and start a new one
    mapping.clear();
}
