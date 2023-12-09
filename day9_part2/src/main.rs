use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
fn main() -> io::Result<()>{

    let start = Instant::now();

    let path = Path::new("day9_part2_input.txt");
    //let path = Path::new("day9_part2_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;
    for line in reader.lines()
    {
        let line = line?;
        let sequence = line.split(' ').collect::<Vec<&str>>();
        //create a int64 vector to hold the numbers
        let mut numbers: Vec<Vec<i64>> = Vec::new(); 
        numbers.push(Vec::new());
        for s in &sequence
        {
            numbers[0].push(s.parse::<i64>().unwrap());
        }
        //println!("numbers: {:?}", numbers);
        let mut all_zero = false;
        let mut current_numbers = &numbers[0];
        while !all_zero 
        {
            //find the differences and add them the next row until they are all 0    
            let mut sub_numbers: Vec<i64> = Vec::new();
            all_zero = true;
            for i in 0..current_numbers.len()-1
            {
                let tmp = current_numbers[i+1] - current_numbers[i];
                sub_numbers.push(tmp);
                if tmp != 0
                {
                    all_zero = false;
                }
            }
            numbers.push(sub_numbers);
            current_numbers = &numbers[numbers.len()-1];
        }
        //println!("numbers: {:?}", numbers);

        //now find the previous value in the first sequence
        for i in (0..numbers.len()-1).rev()
        {
            let tmp = numbers.pop().unwrap();
            let first_num = numbers[i][0];
            let previous_num = first_num - tmp[0];
            //println!("  next_num: {}", next_num);
            numbers[i].insert(0, previous_num);
            //println!("  numbers: {:?}", numbers);
        }
        //get first value of numbers
        let first = numbers.pop().unwrap()[0];
        //println!("last: {:?}", last);
        total += first ;
    }
    println!("total: {}", total);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}