use std::{collections::HashSet, hash::Hash};

use anyhow::{anyhow, Result};
use ndarray::{Array, Array2};

advent_of_code::solution!(10);

const TRAILHEAD: u8 = 0;
const TRAILEND: u8 = 9;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Map {
    width: usize,
    height: usize,
    arr: Array2<u8>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Node<'a> {
    x: usize,
    y: usize,
    val: u8,
    map: &'a Map,
}

impl<'a> Node<'a> {
    fn explore(self) -> HashSet<Node<'a>> {
        let mut set = HashSet::<Node<'a>>::new();

        if self.val == TRAILEND {
            set.insert(self);
            return set;
        }

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (delta_y, delta_x) in dirs {
            let (new_y, new_x) = (
                self.y.checked_add_signed(delta_y),
                self.x.checked_add_signed(delta_x),
            );

            if let (Some(new_y), Some(new_x)) = (new_y, new_x) {
                if let Some(&v) = self.map.arr.get((new_y, new_x)) {
                    if v == self.val + 1 {
                        let node = Node {
                            x: new_x,
                            y: new_y,
                            val: v,
                            map: self.map,
                        };
                        set.extend(node.explore());
                    }
                }
            }
        }

        set
    }

    fn explore_all(self) -> usize {
        let mut res = 0;

        if self.val == TRAILEND {
            return 1;
        }

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (delta_y, delta_x) in dirs {
            let (new_y, new_x) = (
                self.y.checked_add_signed(delta_y),
                self.x.checked_add_signed(delta_x),
            );

            if let (Some(new_y), Some(new_x)) = (new_y, new_x) {
                if let Some(&v) = self.map.arr.get((new_y, new_x)) {
                    if v == self.val + 1 {
                        let node = Node {
                            x: new_x,
                            y: new_y,
                            val: v,
                            map: self.map,
                        };
                        res += node.explore_all();
                    }
                }
            }
        }

        res
    }
}

impl Map {
    fn parse(input: &str) -> Result<Self> {
        let height = input.lines().count();
        let width = input
            .find('\n')
            .ok_or(anyhow!("Did not find more than 1 line in input"))?;
        let arr: Result<Vec<u8>> = input
            .chars()
            .filter(|x| *x != '\n')
            .map(|x| Ok(x.to_digit(10).ok_or(anyhow!("Failed to parse {}", x))? as u8))
            .collect();
        let arr = Array::from_shape_vec((height, width), arr?)?;
        Ok(Map { width, height, arr })
    }

    fn get_trailheads(&self) -> Result<Vec<Node>> {
        let mut res = vec![];
        for y in 0..self.arr.dim().0 {
            for x in 0..self.arr.dim().1 {
                if let Some(&TRAILHEAD) = self.arr.get((y, x)) {
                    res.push(Node {
                        x,
                        y,
                        val: 0,
                        map: &self,
                    });
                }
            }
        }
        Ok(res)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::parse(input).unwrap();
    let trailheads = map.get_trailheads().unwrap();
    let reached: usize = trailheads.into_iter().map(|t| t.explore().len()).sum();
    Some(reached)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::parse(input).unwrap();
    let trailheads = map.get_trailheads().unwrap();
    let reached: usize = trailheads.into_iter().map(|t| t.explore_all()).sum();
    Some(reached)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
