use std::{collections::HashSet, hash::Hash};

use anyhow::{anyhow, Result};
use pathfinding::{
    directed::bfs,
    prelude::{bfs, dfs, dijkstra},
};
use regex::Regex;

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Map {
    walls: HashSet<Position>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str, width: usize, height: usize, fallen: usize) -> Result<Self> {
        let mut walls = HashSet::<Position>::new();
        let re = Regex::new(r"(\d+),(\d+)$")?;
        for l in input.lines().take(fallen) {
            let captures = re.captures(l).ok_or(anyhow!("Failed to parse position"))?;
            let x = captures
                .get(1)
                .and_then(|x| x.as_str().parse().ok())
                .ok_or(anyhow!("Failed to parse x"))?;
            let y = captures
                .get(2)
                .and_then(|y| y.as_str().parse().ok())
                .ok_or(anyhow!("Failed to parse y"))?;
            walls.insert(Position { x, y });
        }
        Ok(Self {
            walls,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell<'a> {
    pos: Position,
    map: &'a Map,
}

impl<'a> Hash for Cell<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl<'a> PartialEq for Cell<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl<'a> Eq for Cell<'a> {}

impl<'a> Cell<'a> {
    fn get_neighbor(&self, dir: &Dir) -> Option<Self> {
        let neighbor = Cell {
            map: self.map,
            pos: match dir {
                Dir::Up => Position {
                    x: self.pos.x,
                    y: self.pos.y.checked_sub(1)?,
                },
                Dir::Right => {
                    let x = self.pos.x.checked_add(1)?;
                    if x >= self.map.width {
                        None?
                    }
                    Position { x, y: self.pos.y }
                }
                Dir::Down => {
                    let y = self.pos.y.checked_add(1)?;
                    if y >= self.map.height {
                        None?
                    }
                    Position { x: self.pos.x, y }
                }
                Dir::Left => Position {
                    x: self.pos.x.checked_sub(1)?,
                    y: self.pos.y,
                },
            },
        };

        if self.map.walls.contains(&neighbor.pos) {
            None
        } else {
            Some(neighbor)
        }
    }

    fn get_neighbors(&self) -> Vec<(Self, usize)> {
        vec![
            self.get_neighbor(&Dir::Up),
            self.get_neighbor(&Dir::Right),
            self.get_neighbor(&Dir::Down),
            self.get_neighbor(&Dir::Left),
        ]
        .into_iter()
        .filter_map(|c_opt| c_opt.and_then(|c| Some((c, 1))))
        .collect()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // let map = Map::parse(input, 71, 71, 1024).unwrap();
    let map = Map::parse(input, 7, 7, 12).unwrap();
    let start = Cell {
        map: &map,
        pos: Position { x: 0, y: 0 },
    };
    let res = dijkstra(
        &start,
        |c| c.get_neighbors(),
        |c| {
            c.pos
                == Position {
                    x: map.width - 1,
                    y: map.height - 1,
                }
        },
    );
    res.and_then(|path| Some(path.1))
}

// #### PART 2 ####
#[derive(Debug)]
struct ChronoMap {
    walls: Vec<Position>,
    width: usize,
    height: usize,
    fallen: usize,
}

impl ChronoMap {
    fn parse(input: &str, width: usize, height: usize) -> Result<Self> {
        let mut walls = vec![];
        let re = Regex::new(r"(\d+),(\d+)$")?;
        for l in input.lines() {
            let captures = re.captures(l).ok_or(anyhow!("Failed to parse position"))?;
            let x = captures
                .get(1)
                .and_then(|x| x.as_str().parse().ok())
                .ok_or(anyhow!("Failed to parse x"))?;
            let y = captures
                .get(2)
                .and_then(|y| y.as_str().parse().ok())
                .ok_or(anyhow!("Failed to parse y"))?;
            walls.push(Position { x, y });
        }
        Ok(Self {
            walls,
            width,
            height,
            fallen: 0,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct ChronoCell<'a> {
    pos: Position,
    map: &'a ChronoMap,
}

impl<'a> Hash for ChronoCell<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl<'a> PartialEq for ChronoCell<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl<'a> Eq for ChronoCell<'a> {}

impl<'a> ChronoCell<'a> {
    fn get_neighbor(&self, dir: &Dir) -> Option<Self> {
        let neighbor = ChronoCell {
            map: self.map,
            pos: match dir {
                Dir::Up => Position {
                    x: self.pos.x,
                    y: self.pos.y.checked_sub(1)?,
                },
                Dir::Right => {
                    let x = self.pos.x.checked_add(1)?;
                    if x >= self.map.width {
                        None?
                    }
                    Position { x, y: self.pos.y }
                }
                Dir::Down => {
                    let y = self.pos.y.checked_add(1)?;
                    if y >= self.map.height {
                        None?
                    }
                    Position { x: self.pos.x, y }
                }
                Dir::Left => Position {
                    x: self.pos.x.checked_sub(1)?,
                    y: self.pos.y,
                },
            },
        };

        let iswall = !neighbor
            .map
            .walls
            .iter()
            .take(neighbor.map.fallen)
            .fold(true, |acc, pos| acc && (pos != &neighbor.pos));

        if iswall {
            None
        } else {
            Some(neighbor)
        }
    }

    fn get_neighbors(&self) -> Vec<Self> {
        vec![
            self.get_neighbor(&Dir::Up),
            self.get_neighbor(&Dir::Right),
            self.get_neighbor(&Dir::Down),
            self.get_neighbor(&Dir::Left),
        ]
        .into_iter()
        .filter_map(|c_opt| c_opt)
        .collect()
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let mut map = ChronoMap::parse(input, 71, 71).unwrap();
    // let mut map = ChronoMap::parse(input, 7, 7).unwrap();
    map.fallen += 1;

    for i in 0..map.walls.len() {
        println!("{}/{}", i, map.walls.len());
        let start = ChronoCell {
            map: &map,
            pos: Position { x: 0, y: 0 },
        };

        let res = dfs(
            start,
            |c| c.get_neighbors(),
            |c| {
                c.pos
                    == Position {
                        x: map.width - 1,
                        y: map.height - 1,
                    }
            },
        );

        if let None = res {
            let wall = &map.walls[i];
            println!("{}", i);
            return Some(format!("{},{}", wall.x, wall.y));
        }

        map.fallen += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
