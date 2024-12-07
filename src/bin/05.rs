use regex::Regex;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

/*
 * Returns a hashmap mapping the page number to a set
 * containing all pages that should be AFTER
 */
fn parse_order(lines: &mut dyn Iterator<Item = &str>) -> Option<HashMap<u32, HashSet<u32>>> {
    let mut res = HashMap::<u32, HashSet<u32>>::new();
    let re = Regex::new(r"(\d*)\|(\d*)").ok()?;

    for l in lines {
        if l.len() == 0 {
            break;
        }

        let captures = re.captures(l)?;
        let p = |x| Some(captures.get(x)?.as_str().parse::<u32>().ok()?);
        let a: u32 = p(1)?;
        let b: u32 = p(2)?;

        match res.get_mut(&a) {
            Some(set) => {
                set.insert(b);
            }
            None => {
                let mut set = HashSet::<u32>::new();
                set.insert(b);
                res.insert(a, set);
            }
        }
    }

    Some(res)
}

/*
 * O(n^2) search to check if manual is correctly indexed
 */
fn check_manual(order: &HashMap<u32, HashSet<u32>>, manual: &Vec<u32>) -> Option<bool> {
    for (i1, p1) in manual[1..].iter().enumerate() {
        if let Some(set) = order.get(p1) {
            for p2 in &manual[..i1 + 1] {
                if set.contains(p2) {
                    return Some(false);
                }
            }
        }
    }
    Some(true)
}

/*
 * None if manual is already sorted, Some(man) with man sorted else
 */
fn sort_manual(order: &HashMap<u32, HashSet<u32>>, mut manual: Vec<u32>) -> Option<Vec<u32>> {
    let mut sorted = true;
    for i in 1..manual.len() {
        for j in 0..i {
            if let Some(set) = order.get(manual.get(i)?) {
                if set.contains(manual.get(j)?) {
                    sorted = false;
                    manual.swap(i, j);
                }
            }
        }
    }
    if sorted {
        None
    } else {
        Some(manual)
    }
}

fn parse_manuals<'a>(lines: impl Iterator<Item = &'a str>) -> Option<Vec<Vec<u32>>> {
    lines
        .map(|l| {
            l.split(',')
                .map(|x| str::parse::<u32>(x).ok())
                .collect::<Option<Vec<u32>>>()
        })
        .collect()
}

fn sum_valid_manuals(
    lines: &mut dyn Iterator<Item = &str>,
    order: HashMap<u32, HashSet<u32>>,
) -> Option<u32> {
    let mut res = 0;
    for l in lines {
        let manual = l
            .split(',')
            .map(|x| str::parse::<u32>(x).ok())
            .collect::<Option<Vec<u32>>>()?;
        res += (check_manual(&order, &manual)? as u32) * manual.get(manual.len() >> 1)?;
    }
    Some(res)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.lines();
    let order = parse_order(&mut input_iter)?;
    sum_valid_manuals(&mut input_iter, order)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_iter = input.lines();
    let order = parse_order(&mut input_iter)?;
    let manuals = parse_manuals(input_iter)?;
    Some(
        manuals
            .into_iter()
            .fold(0, |acc, m| match sort_manual(&order, m) {
                Some(manual) => acc + manual.get(manual.len() >> 1).unwrap(),
                None => acc,
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
