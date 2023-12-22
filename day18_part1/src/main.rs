use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
fn main() -> io::Result<()>{

    let path = Path::new("day18_part1_input.txt");
    //let path = Path::new("day18_part1_test_input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut vertices: Vec<(i64,i64)> = Vec::new();
    let mut edge_colors: Vec<String> = Vec::new();
    let mut current: (i64, i64) = (0,0);
    let mut perimeter = 0;
    let start = Instant::now();
    vertices.push(current);
    let part2 = true;
    for line in reader.lines() 
    {
        let line = line.unwrap();
        let sub_lines: Vec<&str> = line.split_whitespace().collect();
        let steps = if part2 {i64::from_str_radix(&sub_lines[2][2..7], 16).unwrap()} else {sub_lines[1].to_string().parse::<i64>().unwrap()};
        perimeter += steps;
        let dir = if part2 {sub_lines[2].chars().nth(7).unwrap()} else {sub_lines[0].chars().nth(0).unwrap()};
        match dir 
        {
            'R' | '0' => 
            {
                current = (current.0, steps + current.1);
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            'L' | '2' => 
            {
                current = (current.0, current.1 - steps);
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            'U' | '3' => 
            {
                current = (current.0 - steps, current.1);
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            'D' | '1' => 
            {
                current = (current.0 + steps, current.1);
                vertices.push(current);
                edge_colors.push(sub_lines[2][2..8].to_string());
            }
            _ => println!("Error, invalid direction {}", dir),
        }
    }
    println!("Vertices: {:?}", vertices);
    println!("Edge Colors: {:?}", edge_colors);
    //vertices.reverse();
    let area = polygon_area(vertices);
    println!("Area: {}", area);
    //picks theorem for interior points of a polylgon
    let interior = area + (perimeter as f64/2.0) + 1.0;
    println!("Interior: {}", interior);
    let mut end = Instant::now();
    println!("Time: {:?}", end.duration_since(start));
    Ok(())
}

fn polygon_area(vertices: Vec<(i64, i64)>) -> f64 {
    let mut sum = 0.0;
    for i in 0..vertices.len()-1 {
        println!("vertices[i]: {:?}", vertices[i]);
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];
        sum += (x1 * y2) as f64 - (x2 * y1) as f64;
    }
    sum.abs() / 2.0
}