use std::{fs::File, io::Read};

use anyhow::Result;
use regex::{Regex, RegexSet};

enum Instruction {
    Do,
    Dont,
    Multiplication(Multiplication),
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(instruction_string: &str) -> std::result::Result<Self, Self::Error> {
        let set = RegexSet::new(&[r"mul\([0-9]+,[0-9]+\)", r"do\(\)", r"don't\(\)"]).unwrap();

        let matches = set.matches(instruction_string);
        if matches.matched(0) {
            Ok(Self::Multiplication(Multiplication::try_from(
                instruction_string,
            )?))
        } else if matches.matched(1) {
            Ok(Self::Do)
        } else if matches.matched(2) {
            Ok(Self::Dont)
        } else {
            Err(format!("No instruction found in string"))
        }
    }
}

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

    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)")?;
    let mut do_mul = true;
    let result: u32 = regex
        .find_iter(&data)
        .filter_map(
            |instruction| match Instruction::try_from(instruction.as_str()).ok()? {
                Instruction::Do => {
                    do_mul = true;
                    None
                }
                Instruction::Dont => {
                    do_mul = false;
                    None
                }
                Instruction::Multiplication(multiplication) => {
                    if do_mul {
                        Some(multiplication.multiply())
                    } else {
                        None
                    }
                }
            },
        )
        .sum();

    println!("Result: {result}");

    Ok(())
}
