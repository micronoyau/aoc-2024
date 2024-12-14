use std::fmt::Display;
use std::mem;

use anyhow::{anyhow, Context, Result};

advent_of_code::solution!(11);

// Single linked-list
struct Element {
    value: u64,
    next: Option<Box<Element>>,
}

impl Element {
    fn parse(input: &str) -> Result<Option<Box<Self>>> {
        let mut res = None;
        for d in input.split(' ').rev() {
            let new = Box::new(Element {
                value: d.parse().context(anyhow!("Unable to parse {}", d))?,
                next: res,
            });
            res = Some(new);
        }
        Ok(res)
    }

    fn blink<'a, 'b>(&'a mut self) -> Option<&'b mut Box<Element>>
    where
        'a: 'b,
    {
        if self.value == 0 {
            self.value = 1;
            return self.next.as_mut();
        }

        let count = f64::log10(self.value as f64).floor() as u32 + 1;
        if count % 2 == 0 {
            let mask = 10u32.pow(count >> 1);
            let msb = self.value / (mask as u64);
            let lsb = self.value % (mask as u64);
            let old_next = mem::replace(
                &mut self.next,
                Some(Box::new(Element {
                    value: lsb,
                    next: None,
                })),
            );
            self.value = msb;
            let _ = mem::replace(&mut self.next.as_mut().unwrap().next, old_next);
            return self.next.as_mut().unwrap().next.as_mut();
        } else {
            self.value *= 2024;
        }

        self.next.as_mut()
    }

    fn count(self) -> u32 {
        let mut res = 1;
        let mut cur = Box::new(self);
        while let Some(next) = cur.next {
            res += 1;
            cur = next;
        }
        res
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.next {
            Some(next) => write!(f, "{} {}", self.value, next),
            None => write!(f, "{}", self.value),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Element::parse(input).unwrap().unwrap();
    for _ in 0..25 {
        let mut cur = &mut head;
        while let Some(next) = cur.blink() {
            cur = next;
        }
    }
    Some(head.count())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
