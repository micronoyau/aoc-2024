use anyhow::{anyhow, Context, Result};
use ndarray::{Array, Array2};
use std::collections::HashSet;
use std::iter::Iterator;

advent_of_code::solution!(6);

const EXPLORED_CELL: char = 'X';
const WALL_CELL: char = '#';
const FREE_CELL: char = '.';

#[derive(PartialEq, Clone, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn char_to_dir(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::Up),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        _ => None,
    }
}

#[derive(PartialEq, Clone, Eq, Hash)]
struct Guard {
    dir: Direction,
    x: usize,
    y: usize,
}

impl Guard {
    pub fn turn(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

#[derive(Clone)]
struct Map {
    guard: Guard,
    arr: Array2<char>,
}

impl Map {
    pub fn parse(input: &str) -> Result<Map> {
        let arr = Array::from_iter(input.chars().filter(|x| *x != '\n'));
        let arr = arr
            .into_shape_with_order((
                input
                    .chars()
                    .fold(0, |acc, elem| acc + ((elem == '\n') as usize)),
                input
                    .lines()
                    .next()
                    .ok_or(anyhow!("No more than 1 line in input..."))?
                    .len(),
            ))
            .context("Bad array shape")?;

        for row in 0..arr.dim().0 {
            for col in 0..arr.dim().1 {
                if let Some(dir) = char_to_dir(arr[[row, col]]) {
                    return Ok(Map {
                        guard: Guard {
                            dir,
                            x: col,
                            y: row,
                        },
                        arr,
                    });
                }
            }
        }

        Err(anyhow!("Guard not found !"))
    }

    /*
     * Execute a single step.
     * Returns false if guard goes out of bound
     */
    fn step(&mut self) -> Result<bool> {
        let (next_y, next_x) = match self.guard.dir {
            Direction::Up => (self.guard.y.wrapping_sub(1), self.guard.x),
            Direction::Right => (self.guard.y, self.guard.x.wrapping_add(1)),
            Direction::Down => (self.guard.y.wrapping_add(1), self.guard.x),
            Direction::Left => (self.guard.y, self.guard.x.wrapping_sub(1)),
        };

        let curr_c = self
            .arr
            .get_mut((self.guard.y, self.guard.x))
            .ok_or(anyhow!("Bad guard position"))?;
        *curr_c = EXPLORED_CELL;

        if let Some(next_c) = self.arr.get((next_y, next_x)) {
            let next_c = *next_c;
            return match next_c {
                FREE_CELL | EXPLORED_CELL => {
                    self.guard.x = next_x;
                    self.guard.y = next_y;
                    return Ok(true);
                }
                WALL_CELL => {
                    self.guard.turn();
                    return Ok(true);
                }
                _ => Err(anyhow!("Unknown char {}", next_c)),
            };
        }

        Ok(false)
    }

    /*
     * Evolve until guard is out of map.
     * Returns the number of explored positions.
     */
    pub fn evolve(&mut self) -> Result<u32> {
        while self.step()? {}
        Ok(self
            .arr
            .iter()
            .fold(0u32, |acc, elem| acc + ((*elem == EXPLORED_CELL) as u32)))
    }

    /*
     * Evolve until guard is out of map OR until a loop is detected.
     * Returns true if loop is detected, false if guard can escape
     */
    pub fn evolve_is_stuck(&mut self) -> Result<bool> {
        let mut record = HashSet::<Guard>::new();
        while self.step()? {
            // Loop detected
            if record.insert(self.guard.clone()) == false {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::parse(input).unwrap();
    Some(map.evolve().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;
    let map = Map::parse(input).unwrap();

    // First evolve
    let mut map_init = map.clone();
    map_init.evolve().unwrap();

    // For each explored position (except initial one), try to put an obstacle
    *map_init.arr.get_mut((map.guard.y, map.guard.x)).unwrap() = FREE_CELL;
    for row in 0..map_init.arr.dim().0 {
        for col in 0..map_init.arr.dim().1 {
            let c = *map_init.arr.get((row, col)).unwrap();
            // If explored cell, try to block the guard and see what happens
            if c == EXPLORED_CELL {
                let mut map_clone = map.clone();
                *map_clone.arr.get_mut((row, col)).unwrap() = WALL_CELL;
                if map_clone.evolve_is_stuck().unwrap() {
                    res += 1;
                }
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
