advent_of_code::solution!(8);
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::ops::{BitXor, Div};
use std::rc::Rc;

const EMPTY_CELL: char = '.';

#[derive(Debug, PartialEq, Eq, Hash)]
struct Map {
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Antenna {
    x: usize,
    y: usize,
    map: Rc<Map>,
}

/*
 * Compute antinode (p1)
 */
impl Div for &Antenna {
    type Output = Option<Antenna>;
    fn div(self, rhs: Self) -> Self::Output {
        if let Some(new_x) = (2 * self.x).checked_sub(rhs.x) {
            if let Some(new_y) = (2 * self.y).checked_sub(rhs.y) {
                if new_x < self.map.width && new_y < self.map.height {
                    return Some(Antenna {
                        x: new_x,
                        y: new_y,
                        map: self.map.clone(),
                    });
                }
            }
        }
        None
    }
}

/*
 * Compute antinodes (p2)
 */
impl BitXor for &Antenna {
    type Output = HashSet<Antenna>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut res = HashSet::<Antenna>::new();

        // Returns None if out of bounds
        let checked_offset = |a: usize, b: usize, m: usize, limit: usize| {
            let a_signed = a as i64;
            let b_signed = b as i64;
            let m_signed = m as i64;
            let res = a_signed + m_signed * (a_signed - b_signed);
            if res >= 0 {
                let res = res as usize;
                if res < limit {
                    return Some(res as usize);
                }
            }
            None
        };

        // Go further and further until we reach end of map
        let mut m = 0;
        while let (Some(new_x), Some(new_y)) = (
            checked_offset(self.x, rhs.x, m, self.map.width),
            checked_offset(self.y, rhs.y, m, self.map.height),
        ) {
            res.insert(Antenna {
                x: new_x,
                y: new_y,
                map: self.map.clone(),
            });
            m += 1;
        }

        res
    }
}

/*
 * Returns a hashmap with :
 * - Key : antenna type (char)
 * - Value : antennas (vector)
 */
fn parse_map(input: &str) -> Result<HashMap<char, Vec<Antenna>>> {
    let mut res = HashMap::<char, Vec<Antenna>>::new();
    let map = Rc::new(Map {
        width: input
            .lines()
            .next()
            .ok_or(anyhow!("Input does not have a single line"))?
            .len(),
        height: input.lines().count(),
    });

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != EMPTY_CELL {
                let a = Antenna {
                    x,
                    y,
                    map: map.clone(),
                };
                match res.get_mut(&c) {
                    Some(antennas) => {
                        antennas.push(a);
                    }
                    None => {
                        res.insert(c, vec![a]);
                    }
                }
            }
        }
    }

    Ok(res)
}

fn compute_antinodes_p1(map: &HashMap<char, Vec<Antenna>>) -> Result<u32> {
    let mut res = HashSet::<Antenna>::new();
    for (_c, antennas) in map.iter() {
        for i in 0..antennas.len() {
            let a1 = antennas.get(i).unwrap();
            for j in (i + 1)..antennas.len() {
                let a2 = antennas.get(j).unwrap();
                if let Some(a) = a1 / a2 {
                    res.insert(a);
                }
                if let Some(a) = a2 / a1 {
                    res.insert(a);
                }
            }
        }
    }
    Ok(res.len() as u32)
}

fn compute_antinodes_p2(map: &HashMap<char, Vec<Antenna>>) -> Result<u32> {
    let mut res = HashSet::<Antenna>::new();
    for (_c, antennas) in map.iter() {
        for i in 0..antennas.len() {
            let a1 = antennas.get(i).unwrap();
            for j in (i + 1)..antennas.len() {
                let a2 = antennas.get(j).unwrap();
                res.extend(a1 ^ a2);
                res.extend(a2 ^ a1);
            }
        }
    }
    Ok(res.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input).unwrap();
    Some(compute_antinodes_p1(&map).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input).unwrap();
    Some(compute_antinodes_p2(&map).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
