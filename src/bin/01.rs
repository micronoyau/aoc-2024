use regex::Regex;
use std::collections::HashMap;
use std::iter::zip;

advent_of_code::solution!(1);

fn parse(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let re = Regex::new(r"(\d*)(\s*)(\d*)\n").ok()?;
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for m in re.captures_iter(input) {
        left.push(m.get(1)?.as_str().parse().ok()?);
        right.push(m.get(3)?.as_str().parse().ok()?);
    }
    Some((left, right))
}

pub fn part_one(input: &str) -> Option<u32> {
    // First load numbers in memory
    let (mut left, mut right) = parse(input)?;

    // Then sort them (using rust std library's driftsort algorithm)
    left.sort();
    right.sort();

    // Finally, compute L1 distance
    let mut dist: u32 = 0;
    for (l, r) in zip(left, right) {
        dist += l.abs_diff(r) as u32;
    }

    Some(dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    // First load numbers in memory
    let (left, right) = parse(input)?;

    // How many times each left number appears in the right list
    let mut count: HashMap<u32, u32> = HashMap::new();
    // Total score for each number
    let mut total: HashMap<u32, u32> = HashMap::new();
    for l in left {
        let c = {
            if let Some(res) = count.get(&l) {
                *res
            } else {
                let mut res = 0;
                for r in right.as_slice() {
                    if r == &l {
                        res += 1;
                    }
                }
                count.insert(l, res);
                res
            }
        };

        match total.get_mut(&l) {
            Some(prev) => {
                *prev += c * l;
            }
            None => {
                total.insert(l, l * c);
            }
        }
    }

    // Sum it up
    Some(total.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
