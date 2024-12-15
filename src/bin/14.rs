use std::fmt::Display;

use anyhow::{anyhow, Result};
use ndarray::Array2;
use regex::Regex;

advent_of_code::solution!(14);

const MAP_SOLVE_WIDTH: usize = 101;
const MAP_SOLVE_HEIGHT: usize = 103;
const MAP_TEST_WIDTH: usize = 11;
const MAP_TEST_HEIGHT: usize = 7;
const EMPTY_TILE: char = '.';

#[derive(Debug)]
struct Robot {
    px: usize,
    py: usize,
    vx: isize,
    vy: isize,
}

#[derive(Debug)]
struct TileMap {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Robot {
    fn parse(line: &str) -> Result<Self> {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;
        let err = |l: &str| anyhow!("Failed to parse {}", l);
        let capture = re.captures(line).ok_or(err(line))?;
        let px = capture.get(1).ok_or(err(line))?.as_str();
        let py = capture.get(2).ok_or(err(line))?.as_str();
        let vx = capture.get(3).ok_or(err(line))?.as_str();
        let vy = capture.get(4).ok_or(err(line))?.as_str();
        Ok(Robot {
            px: px.parse().map_err(|_| err(px))?,
            py: py.parse().map_err(|_| err(py))?,
            vx: vx.parse().map_err(|_| err(vx))?,
            vy: vy.parse().map_err(|_| err(vy))?,
        })
    }

    fn step(&mut self, width: usize, height: usize) {
        // Quick and dirty hack : assume that displacement < boundaries
        self.px = self
            .px
            .checked_add_signed(width as isize + self.vx)
            .unwrap()
            % width;
        self.py = self
            .py
            .checked_add_signed(height as isize + self.vy)
            .unwrap()
            % height;
    }
}

impl TileMap {
    fn parse(input: &str, width: usize, height: usize) -> Result<Self> {
        let mut robots = vec![];
        for l in input.lines() {
            robots.push(Robot::parse(l)?);
        }
        Ok(Self {
            width,
            height,
            robots,
        })
    }

    fn step(&mut self) {
        for r in &mut self.robots {
            r.step(self.width, self.height);
        }
    }

    fn safety_factor(&self) -> usize {
        let hw = self.width >> 1;
        let hh = self.height >> 1;
        let mut nw = 0;
        let mut ne = 0;
        let mut sw = 0;
        let mut se = 0;
        for r in &self.robots {
            nw += ((r.px < hw) && (r.py < hh)) as usize;
            ne += ((r.px < hw) && (r.py > hh)) as usize;
            sw += ((r.px > hw) && (r.py < hh)) as usize;
            se += ((r.px > hw) && (r.py > hh)) as usize;
        }
        nw * ne * sw * se
    }
}

impl Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = Array2::from_elem((self.height, self.width), 0);
        for r in &self.robots {
            let c = map.get_mut((r.py, r.px)).unwrap();
            *c += 1;
        }
        let map = map.map(|n| {
            if *n == 0 {
                return Ok(EMPTY_TILE);
            }
            n.to_string()
                .chars()
                .next()
                .ok_or(anyhow!("Failed to parse to char {}", n))
        });
        let map: Result<Vec<char>> = map.into_iter().collect();
        let map = map.map_err(|_| std::fmt::Error)?;
        let map =
            Array2::from_shape_vec((self.height, self.width), map).map_err(|_| std::fmt::Error)?;
        for l in map.rows() {
            writeln!(f, "{}", l.iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut tilemap = {
        if input.lines().count() < 13 {
            TileMap::parse(input, MAP_TEST_WIDTH, MAP_TEST_HEIGHT)
        } else {
            TileMap::parse(input, MAP_SOLVE_WIDTH, MAP_SOLVE_HEIGHT)
        }
    }
    .unwrap();
    // println!("{}", tilemap);

    for _ in 0..100 {
        tilemap.step();
    }
    // println!("{}", tilemap);

    Some(tilemap.safety_factor())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tilemap = TileMap::parse(input, MAP_SOLVE_WIDTH, MAP_SOLVE_HEIGHT).unwrap();
    println!("{}", tilemap);

    // Since the picture is assumed to be concentrated in one point,
    // it should have a small safety factor !
    let mut min_safety_factor = tilemap.safety_factor();
    let mut min_index = 0;
    for i in 1..20000 {
        tilemap.step();
        let safety_factor = tilemap.safety_factor();
        if safety_factor < min_safety_factor {
            min_index = i;
            min_safety_factor = safety_factor;
            println!("{}", tilemap);
        }
    }

    Some(min_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
