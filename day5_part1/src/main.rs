use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()>{
    //let path = Path::new("day5_part1_input.txt");
    let path = Path::new("day5_part1_input.txt");
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
        for part in line_parts[1..].iter()
        {
            println!("part: {}", part);
            seed_ids.push(part.parse::<u64>().unwrap());
        }
    }

    let mut mapping: Vec<(u64, u64, u64)> = Vec::new();

    //read blank and then read through next blank line & the map name
    reader.read_line(&mut line)?;
    for line in reader.lines()
    {
        let line = line?;
        //print line with hidden chars visible
        println!("line: {:?}", line);
        //check for next map
        if line == ""
        {
            println!("Found newline");
            //use the current map to do the mapping
            assert!(mapping.len() > 0);
            update_mapping(&mut mapping, &mut seed_ids);
        }
        else if line.contains("map")
        {
            println!("Found map string");
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
    println!("Final mapping");
    update_mapping(&mut mapping, &mut seed_ids);

    //sort seed_ids and get the lowest one
    seed_ids.sort();
    println!("lowest seed id: {}", seed_ids[0]);
    //print last seed_id
    println!("highest seed id: {}", seed_ids[seed_ids.len() - 1]);
    Ok(())
}

fn update_mapping(mapping: &mut Vec<(u64, u64, u64)>, seed_ids: &mut Vec<u64>) {
    println!("In mapping with mapping len: {}", mapping.len()); 
    //sort the mapping by the first element
    mapping.sort_by(|a, b| a.0.cmp(&b.0));
    //for each seed id, do the mapping
    for seed_id in seed_ids
    {
        //go through the mapping and find the appropriate range if one exist
        for map in &*mapping
        {   
            println!(" checking seed_id: {} with map: {:?}", *seed_id, map);
            if *seed_id >= map.0 && *seed_id < map.0 + map.2
            {
                println!("found range");
                let offset = *seed_id - map.0;
                let new_id = map.1 + offset;
                println!("mapping seed_id: {} to new_id: {}", *seed_id, new_id);
                *seed_id = new_id;
                break;
            }
        }
        //since default mapping is 1:1, we don't need to do anything if no range is found 
    }
    //clear the map and start a new one
    mapping.clear();
}
