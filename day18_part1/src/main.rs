use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
fn main() -> io::Result<()>{

    //let path = Path::new("day18_part1_input.txt");
    let path = Path::new("day18_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut vertices: Vec<(i32,i32)> = Vec::new();
    let mut edge_colors: Vec<String> = Vec::new();
    let mut current = (0,0);
    let start = Instant::now();
    vertices.push(current);
    for line in reader.lines() 
    {
        let line = line.unwrap();
        let sub_lines: Vec<&str> = line.split_whitespace().collect();
        match sub_lines[0]
        {
            "R" => 
            {
                current = (current.0, sub_lines[1].to_string().parse::<i32>().unwrap() + current.1);
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            "L" => 
            {
                current = (current.0, current.1 - sub_lines[1].to_string().parse::<i32>().unwrap());
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            "U" => 
            {
                current = (current.0 - sub_lines[1].to_string().parse::<i32>().unwrap(), current.1);
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            "D" => 
            {
                current = (current.0 + sub_lines[1].to_string().parse::<i32>().unwrap(), current.1);
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            _ => println!("Error")
        }
    }
    println!("Vertices: {:?}", vertices);
    println!("Edge Colors: {:?}", edge_colors);
    //vertices.reverse();
    let ans = polygon_area(vertices);
    println!("Area: {}", ans);

    let mut end = Instant::now();
    println!("Time: {:?}", end.duration_since(start));
    Ok(())
}

fn polygon_area(vertices: Vec<(i32, i32)>) -> f64 {
    let mut sum = 0.0;
    for i in 0..vertices.len()-1 {
        println!("vertices[i]: {:?}", vertices[i]);
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];
        sum += (x1 * y2) as f64 - (x2 * y1) as f64;
    }
    sum.abs() / 2.0
}