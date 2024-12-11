use anyhow::{anyhow, Result};
use std::{collections::HashSet, fs::File, io::Read, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum GridElement {
    Obstacle,
    Empty,
    Guard(Direction),
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_90(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

struct Guard {
    location: Coordinate,
    direction: Direction,
}

struct Grid {
    grid: Vec<Vec<GridElement>>,
    guard: Option<Guard>,
}

impl Grid {
    fn new(data: String) -> Result<Self, ParseGridElementError> {
        let mut guard = None;
        let grid = data
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        let grid_element = char.to_string().parse::<GridElement>();

                        if let Ok(GridElement::Guard(direction)) = grid_element {
                            guard = Some(Guard {
                                location: Coordinate { x, y },
                                direction,
                            });

                            Ok(GridElement::Empty)
                        } else {
                            grid_element
                        }
                    })
                    .collect::<Result<Vec<GridElement>, ParseGridElementError>>()
            })
            .collect::<Result<Vec<Vec<GridElement>>, ParseGridElementError>>();

        match grid {
            Ok(grid) => Ok(Self { grid, guard }),
            Err(e) => Err(e),
        }
    }

    fn look_around(
        &self,
        coordinate: &Coordinate,
        direction: &Direction,
    ) -> Option<(GridElement, Coordinate)> {
        let x = coordinate.x;
        let y = coordinate.y;

        let new_coordinate = match direction {
            Direction::Up => {
                let y = y as isize - 1;

                if y < 0 {
                    None
                } else {
                    Some(Coordinate { x, y: y as usize })
                }
            }
            Direction::Down => {
                let y = y + 1;

                if y >= self.grid.len() {
                    None
                } else {
                    Some(Coordinate { x, y })
                }
            }
            Direction::Left => {
                let x = x as isize - 1;

                if x < 0 {
                    None
                } else {
                    Some(Coordinate { x: x as usize, y })
                }
            }
            Direction::Right => {
                let x = x + 1;

                if x >= self.grid[y as usize].len() {
                    None
                } else {
                    Some(Coordinate { x, y })
                }
            }
        };

        if let Some(coordinate) = new_coordinate {
            Some((self.grid[coordinate.y][coordinate.x], coordinate))
        } else {
            None
        }
    }

    fn move_guard(self) -> Self {
        if let Some(guard) = &self.guard {
            let guard = match self.look_around(&guard.location, &guard.direction) {
                Some((GridElement::Obstacle, _)) => Some(Guard {
                    location: guard.location,
                    direction: guard.direction.rotate_90(),
                }),
                Some((GridElement::Empty, coordinate)) => Some(Guard {
                    location: coordinate,
                    direction: guard.direction,
                }),
                None => None,
                _ => unreachable!(),
            };

            return Self {
                grid: self.grid,
                guard: guard,
            };
        } else {
            return self;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGridElementError;
impl std::fmt::Display for ParseGridElementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable convert string to Grid Element")
    }
}

impl FromStr for GridElement {
    type Err = ParseGridElementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Obstacle),
            "." => Ok(Self::Empty),
            "^" => Ok(Self::Guard(Direction::Up)),
            ">" => Ok(Self::Guard(Direction::Right)),
            "V" => Ok(Self::Guard(Direction::Down)),
            "<" => Ok(Self::Guard(Direction::Left)),
            _ => Err(ParseGridElementError),
        }
    }
}
fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut data_string = String::new();
    file.read_to_string(&mut data_string)?;

    let mut grid = Grid::new(data_string).map_err(|_| anyhow!("Couldn't create grid"))?;

    let mut spaces: HashSet<Coordinate> = HashSet::new();

    while let Some(ref guard) = grid.guard {
        spaces.insert(guard.location);
        grid = grid.move_guard();
    }

    let result = spaces.len();

    println!("Result: {result}");

    Ok(())
}
