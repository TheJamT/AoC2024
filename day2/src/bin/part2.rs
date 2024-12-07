use std::{fs::File, io::Read};

use anyhow::Result;

fn find_safe(data: Vec<i32>) -> bool {
    !data
        .iter()
        .zip(data.iter().skip(1))
        .map(|(a, b)| {
            if (((*a > 0) && (*b > 0)) || ((*a < 0) && (*b < 0)))
                && a.abs() <= 3
                && b.abs() <= 3
                && a.abs() >= 1
                && b.abs() >= 1
            {
                true
            } else {
                false
            }
        })
        .any(|i| !i)
}

fn main() -> Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut data_string = String::new();
    let _ = file.read_to_string(&mut data_string);

    let data: Vec<Vec<i32>> = data_string
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let result = data.iter().fold(0, |acc, report| {
        let changes = report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<i32>>();

        if find_safe(changes.clone()) {
            acc + 1
        } else {
            let safe = report
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let mut cloned_report = report.clone();
                    cloned_report.remove(i);

                    let changes = cloned_report
                        .iter()
                        .zip(cloned_report.iter().skip(1))
                        .map(|(a, b)| b - a)
                        .collect::<Vec<i32>>();

                    find_safe(changes)
                })
                .any(|i| i);

            if safe {
                acc + 1
            } else {
                acc
            }
        }
    });

    println!("{result}");

    Ok(())
}
