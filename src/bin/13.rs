use anyhow::{anyhow, Error, Result};
use ndarray::{arr2, Array};
use regex::Regex;
use std::iter::zip;

advent_of_code::solution!(13);

const A_COST: u64 = 3;
const B_COST: u64 = 1;

#[derive(Debug)]
struct Offset {
    x: u64,
    y: u64,
}

impl TryFrom<(&str, &Regex)> for Offset {
    type Error = Error;
    fn try_from(value: (&str, &Regex)) -> std::result::Result<Self, Self::Error> {
        let line = value.0;
        let re = value.1;
        let err = |l: &str| anyhow!("Failed to parse {}", l);
        let capture = re.captures(line).ok_or(err(line))?;
        let x = capture.get(1).ok_or(err(line))?.as_str();
        let x: u64 = x.parse().map_err(|_| err(x))?;
        let y = capture.get(2).ok_or(err(line))?.as_str();
        let y: u64 = y.parse().map_err(|_| err(y))?;
        Ok(Offset { x, y })
    }
}

#[derive(Debug)]
struct Machine {
    a: Offset,
    b: Offset,
    prize: Offset,
}

impl Machine {
    fn parse(input: &str) -> Result<Vec<Machine>> {
        let mut res = vec![];
        let iter = zip(input.lines().step_by(4), input.lines().skip(1).step_by(4));
        let iter = zip(iter, input.lines().skip(2).step_by(4));
        let re_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?;
        let re_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?;
        let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;
        for ((a, b), prize) in iter {
            res.push(Machine {
                a: (a, &re_a).try_into()?,
                b: (b, &re_b).try_into()?,
                prize: (prize, &re_prize).try_into()?,
            });
        }
        Ok(res)
    }

    /*
     * Use Gauss pivot to solve linear system
     */
    fn solve(&self) -> Option<u64> {
        let mut system = arr2(&[
            [self.a.x, self.b.x, self.prize.x],
            [self.a.y, self.b.y, self.prize.y],
        ])
        .map(|x| *x as i64);

        // Eliminate bottom left term
        let c0 = -*system.get((1, 0)).unwrap();
        let c1 = *system.get((0, 0)).unwrap();
        for j in 0..system.dim().1 {
            let r0 = *system.get((0, j)).unwrap();
            let r1 = system.get_mut((1, j)).unwrap();
            *r1 = c0 * r0 + c1 * *r1;
        }

        // Attempt to normalize last row
        let q = *system.get((1, 1)).unwrap();
        let p = system.get_mut((1, 2)).unwrap();
        if p.checked_rem(q)? != 0 {
            return None;
        }
        *p = *p / q;
        *system.get_mut((1, 1)).unwrap() = 1;

        // Eliminate top right term
        let c0 = 1;
        let c1 = -*system.get((0, 1)).unwrap();
        for j in 0..system.dim().1 {
            let r1 = *system.get((1, j)).unwrap();
            let r0 = system.get_mut((0, j)).unwrap();
            *r0 = c0 * *r0 + c1 * r1;
        }

        // Attempt to normalize first row
        let q = *system.get((0, 0)).unwrap();
        let p = system.get_mut((0, 2)).unwrap();
        if p.checked_rem(q)? != 0 {
            return None;
        }
        *p = *p / q;
        *system.get_mut((0, 0)).unwrap() = 1;

        // Try to convert
        let a = *system.get((0, 2)).unwrap();
        let b = *system.get((1, 2)).unwrap();
        if (a < 0) | (b < 0) {
            return None;
        }
        let a = a as u64;
        let b = b as u64;

        Some(a * A_COST + b * B_COST)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = Machine::parse(input).unwrap();
    Some(machines.iter().fold(0, |acc, m| match m.solve() {
        Some(val) => acc + val,
        None => acc,
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut machines = Machine::parse(input).unwrap();
    for m in &mut machines {
        m.prize.x += 10000000000000u64;
        m.prize.y += 10000000000000u64;
    }
    Some(machines.iter().fold(0, |acc, m| match m.solve() {
        Some(val) => acc + val,
        None => acc,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908)); // Had to compute it on my own
    }
}
