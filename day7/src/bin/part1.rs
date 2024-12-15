use anyhow::Result;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut string = String::new();
    let _ = file.read_to_string(&mut string)?;

    let result: u64 = string
        .lines()
        .filter_map(|line| {
            let mut line = line.split(':');

            let answer = line.next().unwrap().parse::<u64>().unwrap();

            let values = line
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let total_true = (0..(2usize.pow(values.len().try_into().unwrap())))
                .filter_map(|permutation| {
                    let values_cloned = values.clone();
                    let output = (0..values.len()).zip(values_cloned).skip(1).fold(
                        values[0],
                        |acc, (shift, value)| match (permutation >> shift & 1) != 0 {
                            true => acc * value,
                            false => acc + value,
                        },
                    );

                    if output == answer {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();

            if total_true > 0 {
                Some(answer)
            } else {
                None
            }
        })
        .sum();

    println!("{result}");

    Ok(())
}
