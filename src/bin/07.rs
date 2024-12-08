use anyhow::{anyhow, Result};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    res: u64,
    terms: Vec<u64>,
}

impl Equation {
    pub fn parse(line: &str) -> Result<Self> {
        let mut l_split = line.split(": ");
        let res = l_split
            .next()
            .ok_or(anyhow!("Failed to parse result"))?
            .parse()?;
        let l_split = l_split
            .next()
            .ok_or(anyhow!("Failed to parse terms"))?
            .split(' ');
        let terms = l_split
            .map(|x| {
                x.parse::<u64>()
                    .map_err(|e| anyhow!("Failed to parse integer : {}", e))
            })
            .collect::<Result<Vec<u64>>>()?;
        Ok(Equation { res, terms })
    }

    fn is_solvable_core_p1(&self, res: u64, ptr: usize, current: u64) -> bool {
        // Small optimization
        if current > res {
            return false;
        }
        match self.terms.get(ptr) {
            Some(&val) => {
                self.is_solvable_core_p1(res, ptr + 1, current * val)
                    || self.is_solvable_core_p1(res, ptr + 1, current + val)
            }
            None => current == res,
        }
    }

    fn is_solvable_core_p2(&self, res: u64, ptr: usize, current: u64) -> bool {
        // Small optimization
        if current > res {
            return false;
        }
        match self.terms.get(ptr) {
            Some(&val) => {
                let val_offset = f64::log10(val as f64) as u32;
                self.is_solvable_core_p2(res, ptr + 1, current * val)
                    || self.is_solvable_core_p2(res, ptr + 1, current + val)
                    || self.is_solvable_core_p2(
                        res,
                        ptr + 1,
                        (current * 10u64.pow(val_offset + 1)) + val,
                    )
            }
            None => current == res,
        }
    }

    pub fn is_solvable_p1(&self) -> bool {
        self.is_solvable_core_p1(self.res, 0, 0)
    }

    pub fn is_solvable_p2(&self) -> bool {
        self.is_solvable_core_p2(self.res, 0, 0)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| Equation::parse(l).unwrap())
            .fold(0u64, |acc, elem| {
                acc + (elem.is_solvable_p1() as u64) * elem.res
            }) as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| Equation::parse(l).unwrap())
            .fold(0u64, |acc, elem| {
                acc + (elem.is_solvable_p2() as u64) * elem.res
            }) as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
