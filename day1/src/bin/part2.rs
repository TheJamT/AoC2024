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

    let result: i32 = left
        .iter()
        .map(|num| {
            let occurances = right
                .iter()
                .fold(0, |acc, x| if x == num { acc + 1 } else { acc });
            occurances * num
        })
        .sum();

    println!("{result}");

    Ok(())
}
