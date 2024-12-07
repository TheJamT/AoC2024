use std::{fs::File, io::Read};

use anyhow::Result;

fn main() -> Result<()> {
    let mut input = File::open("./input.txt")?;

    let mut input_str = String::new();
    input.read_to_string(&mut input_str)?;

    let data: Vec<(i32, i32)> = input_str
        .split("\n")
        .filter_map(|line| {
            let mut split_line = line.split_whitespace();

            Some((
                split_line.next()?.parse().ok()?,
                split_line.next()?.parse().ok()?,
            ))
        })
        .collect();

    let mut left = Vec::new();
    let mut right = Vec::new();

    data.iter().for_each(|datum| {
        left.push(datum.0);
        right.push(datum.1);
    });

    left.sort();
    right.sort();

    let result: i32 = left
        .iter()
        .zip(right)
        .map(|(left, right)| (right - left).abs())
        .sum();

    println!("{result}");

    Ok(())
}
