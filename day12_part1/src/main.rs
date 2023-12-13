use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use std::collections::HashMap;
//46.89 seconds debug part 1
//45.36 seconds debug part 1

fn main() -> io::Result<()>{

    let start = Instant::now();

    //let path = Path::new("day12_part1_input.txt");
    let path = Path::new("day12_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut hot_springs: String;
    let mut condition: Vec<usize>; 
    let mut total_combinations = 0;
    let part1 = true;
    let mut cache: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let use_dp = true;
    for line in reader.lines()
    {
        let line = line?;
        let sub_strings: Vec<&str> = line.split(' ').collect(); 
        hot_springs = sub_strings[0].to_string(); //.chars().collect();
        condition = sub_strings[1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        if part1
        {
            if use_dp
            {
                let mut cache2: HashMap<(usize, usize, usize), u64> = HashMap::new();
                let tmp_combination = compute_dp(&hot_springs, &condition, 0, 0, 0, &mut cache2);
                println!("Combination: {}", tmp_combination);
                total_combinations += tmp_combination;
            }
            else 
            {
                let tmp_combination = compute(&hot_springs, &condition, &mut cache);
                println!("Combination: {}", tmp_combination);
                total_combinations += tmp_combination;
            }
        }
        else
        {
            //part2
            //concat hot_springs with itself 4 times with '?' in between
            let mut tmp_hot_springs = hot_springs.clone();
            for _ in 0..4
            {
                tmp_hot_springs.push('?');
                tmp_hot_springs.push_str(&hot_springs);
            }
            //concat 4 copies of condition
            let mut tmp_condition = condition.clone();
            for _ in 0..4
            {
                tmp_condition.append(&mut condition.clone());
            }
            if use_dp
            {
                let mut cache2: HashMap<(usize, usize, usize), u64> = HashMap::new();
                let tmp_combination = compute_dp(&tmp_hot_springs, &tmp_condition, 0, 0, 0, &mut cache2);
                println!("Combination: {}", tmp_combination);
                total_combinations += tmp_combination;
            }
            else
            {
                let tmp_combination = compute(&tmp_hot_springs, &tmp_condition, &mut cache);
                println!("Combination: {}", tmp_combination);
                total_combinations += tmp_combination;
            }
        }
    }  

    println!("Total combinations: {}", total_combinations); 
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}

fn compute(hot_springs: &str, condition: &Vec<usize>, cache: &mut HashMap<String, HashMap<String, u64>>) -> u64
{ 
    let condition_str = format!("{:?}", condition);
    
    if let Some(inner_map) = cache.get(hot_springs) {
        if let Some(&result) = inner_map.get(&condition_str) {
            println!("Cache hit for hot_springs: {}, condition: {}", hot_springs, condition_str);
            return result;
        }
    }

    let mut count = 0;

    if hot_springs.contains('?')
    {
        //let mut tmp_condition = condition.clone();
        let tmp_hot_springs1 = hot_springs.replacen("?", ".", 1);
        let tmp1 = compute(&tmp_hot_springs1, condition, cache);
        count += tmp1;

        //check if cache contains tmp_hot_springs1, if true then insert condition in to its value
        //else insert tmp_hot_springs1 and condition in to cache
        cache.entry(tmp_hot_springs1)
            .or_insert_with(HashMap::new)
            .insert(condition_str.clone(), tmp1);

        let tmp_hot_springs2 = hot_springs.replacen("?", "#", 1);
        let tmp2 = compute(&tmp_hot_springs2.clone(), condition, cache);
        count += tmp2;
        
        cache.entry(tmp_hot_springs2)
            .or_insert_with(HashMap::new)
            .insert(condition_str.clone(), tmp2);
        
        return count;
    }
    else
    {
        let groups: Vec<usize> = hot_springs.split('.')
                .filter(|group| !group.is_empty())
                .map(|group| group.chars().filter(|&c| c == '#').count())
                .collect();
        if groups == *condition
        {
            return 1;
        }
        else
        {
            return 0;
        }
    }
}

//needed help with this and referenced https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
fn compute_dp(hot_springs: &str, condition: &Vec<usize>, i: usize, bi: usize, current: usize, cache: &mut HashMap<(usize, usize, usize), u64>) -> u64
{
    if let Some(&result) = cache.get(&(i, bi, current)) {
        println!("Cache hit for hot_springs: {}, condition: {}", hot_springs, format!("{:?}", condition));
        return result;
    }

    if i == hot_springs.len()
    {
        if bi == condition.len() && current == 0
        {
            return 1;
        }
        else if bi == condition.len()-1 && condition[bi] == current
        {
            return 1;
        }
        else
        {
            return 0;
        }
    }
    let mut ans: u64 = 0;
    for c in ['.', '#'].iter()
    {
        if hot_springs.chars().nth(i).unwrap() == *c || hot_springs.chars().nth(i).unwrap() == '?'
        {
            if *c == '.' && current == 0
            {
                ans += compute_dp(hot_springs, condition, i+1, bi, 0, cache);
            }
            else if *c == '.' && current > 0 && bi < condition.len() && condition[bi] == current
            {
                ans += compute_dp(hot_springs, condition, i+1, bi+1, 0, cache);
            }
            else if *c == '#'
            {
                ans += compute_dp(hot_springs, condition, i+1, bi, current+1, cache);
            }
        }
    }

    //insert in to cache
    cache.insert((i, bi, current), ans);
    
    ans
}


