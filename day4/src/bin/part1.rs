use std::{fs::File, io::Read};

use anyhow::{anyhow, Result};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Letters {
    X,
    M,
    A,
    S,
}

#[derive(EnumIter)]
enum Direction {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

#[derive(Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl TryFrom<&str> for Letters {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if value.len() != 1 {
            return Err(format!("String provided is incorrect length"));
        }
        match value {
            "X" => Ok(Letters::X),
            "M" => Ok(Letters::M),
            "A" => Ok(Letters::A),
            "S" => Ok(Letters::S),
            _ => Err(format!("String does match value X M A S: {value}")),
        }
    }
}

fn find_coordinate(data: &Vec<Vec<Letters>>, x: isize, y: isize) -> Option<(Letters, Coordinate)> {
    if x < 0 || y < 0 {
        return None;
    }

    let x = match usize::try_from(x) {
        Ok(x) => x,
        Err(_) => unreachable!(),
    };

    let y = match usize::try_from(y) {
        Ok(y) => y,
        Err(_) => unreachable!(),
    };

    Some((data.get(y)?.get(x)?.clone(), (x, y).into()))
}

fn go_direction(
    data: &Vec<Vec<Letters>>,
    direction: &Direction,
    start_coordinates: Coordinate,
) -> Option<(Letters, Coordinate)> {
    let x = start_coordinates.x as isize;
    let y = start_coordinates.y as isize;

    match direction {
        Direction::NW => find_coordinate(data, x - 1, y - 1),
        Direction::N => find_coordinate(data, x, y - 1),
        Direction::NE => find_coordinate(data, x + 1, y - 1),
        Direction::E => find_coordinate(data, x + 1, y),
        Direction::SE => find_coordinate(data, x + 1, y + 1),
        Direction::S => find_coordinate(data, x, y + 1),
        Direction::SW => find_coordinate(data, x - 1, y + 1),
        Direction::W => find_coordinate(data, x - 1, y),
    }
}

fn find_m(data: &Vec<Vec<Letters>>, coordinate: Coordinate) -> Vec<(Direction, Coordinate)> {
    Direction::iter()
        .filter_map(|direction| {
            if let Some((letter, coordinate)) = go_direction(data, &direction, coordinate) {
                if letter == Letters::M {
                    Some((direction, coordinate))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    let _ = file.read_to_string(&mut data)?;

    let data = data
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| {
                    Letters::try_from(char.to_string().as_str()).map_err(|e| anyhow!("{e}"))
                })
                .collect::<Result<Vec<Letters>>>()
        })
        .collect::<Result<Vec<Vec<Letters>>>>()?;

    let result = data.iter().enumerate().fold(0, |row_acc, (y, row)| {
        let col_acc = row.iter().enumerate().fold(0, |col_acc, (x, cell)| {
            let coordinate = (x, y).into();

            if *cell != Letters::X {
                return col_acc;
            }

            let num_matches = find_m(&data, coordinate)
                .iter()
                .filter_map(|(match_direction, m_match_coordinate)| {
                    if let Some((letter, a_match_coordinate)) =
                        go_direction(&data, match_direction, *m_match_coordinate)
                    {
                        if letter == Letters::A {
                            if let Some((letter, _)) =
                                go_direction(&data, match_direction, a_match_coordinate)
                            {
                                if letter == Letters::S {
                                    return Some(());
                                }
                            }
                        }
                    }

                    return None;
                })
                .collect::<Vec<()>>()
                .len();

            col_acc + num_matches
        });

        row_acc + col_acc
    });

    println!("{result}");

    Ok(())
}
