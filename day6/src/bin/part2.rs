use anyhow::{anyhow, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashSet, fs::File, io::Read, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridElement {
    Obstacle,
    Empty,
    Guard(Direction),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    location: Coordinate,
    direction: Direction,
}

#[derive(Clone)]
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

    fn change_cell(&mut self, coordinate: Coordinate, element: GridElement) {
        self.grid[coordinate.y][coordinate.x] = element;
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

    let grid = Grid::new(data_string).map_err(|_| anyhow!("Couldn't create grid"))?;
    let starting_cell = grid.guard.ok_or(anyhow!("No starting guard position"))?;

    let mut original_moves: HashSet<Coordinate> = HashSet::new();
    let mut test_grid = grid.clone();

    while let Some(ref guard) = test_grid.guard {
        original_moves.insert(guard.location);
        test_grid = test_grid.move_guard();
    }

    let result = original_moves
        .par_iter()
        .filter_map(|cell| {
            if *cell != starting_cell.location {
                let mut duplicate_grid = grid.clone();
                duplicate_grid.change_cell(*cell, GridElement::Obstacle);

                duplicate_grid = duplicate_grid.move_guard();
                let mut positions: HashSet<Guard> = HashSet::new();

                while let Some(ref guard) = duplicate_grid.guard {
                    if !positions.insert(*guard) {
                        return Some(());
                    }

                    duplicate_grid = duplicate_grid.move_guard()
                }
                None
            } else {
                None
            }
        })
        .count();

    println!("Result: {result}");

    Ok(())
}
