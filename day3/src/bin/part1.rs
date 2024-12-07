use std::{fs::File, io::Read};

use anyhow::Result;
use regex::Regex;

struct Multiplication {
    x: u16,
    y: u16,
}

impl TryFrom<&str> for Multiplication {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let re = Regex::new(r"[0-9]+,[0-9]+").unwrap();

        let num_string = re
            .find(&value)
            .ok_or(format!("Couldn't find numbers in Mul string"))?;
        let mut split = num_string.as_str().split(",");

        let x = split
            .next()
            .ok_or(format!("Couldn't find x"))?
            .parse()
            .map_err(|e| format!("Couldn't parse x: {e}"))?;
        let y = split
            .next()
            .ok_or(format!("Couldn't find y"))?
            .parse()
            .map_err(|e| format!("Couldn't parse yL {e}"))?;

        Ok(Self { x, y })
    }
}

impl Multiplication {
    fn multiply(self) -> u32 {
        self.x as u32 * self.y as u32
    }
}

fn main() -> Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut data = String::new();

    let _ = file.read_to_string(&mut data)?;

    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)")?;
    let result: u32 = regex
        .find_iter(&data)
        .filter_map(|mul| Some(Multiplication::try_from(mul.as_str()).ok()?.multiply()))
        .sum();

    println!("Result: {result}");

    Ok(())
}
