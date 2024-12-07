use anyhow::{anyhow, Context, Result};
use ndarray::{Array, Array2};
use regex::Regex;
use strum::{EnumIter, IntoEnumIterator};

advent_of_code::solution!(4);
const DELIMITER: char = '\n';

#[derive(Debug, EnumIter, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    DiagUpRight,
    DiagUpLeft,
    DiagDownRight,
    DiagDownLeft,
}

struct Explorer<'a> {
    dir: Direction,
    x: i32,
    y: i32,
    input: &'a Array2<char>,
}

impl<'a> Explorer<'a> {
    pub fn new(dir: Direction, input: &'a Array2<char>) -> Result<Self> {
        let mut x = 0;
        let mut y = 0;

        match dir {
            Direction::Left => x = (input.dim().1 - 1) as i32,
            Direction::Up | Direction::DiagDownRight | Direction::DiagUpLeft => {
                y = (input.dim().0 - 1) as i32
            }
            _ => {}
        }

        Ok(Explorer { dir, x, y, input })
    }
}

impl<'a> Iterator for Explorer<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.input.get((self.y as usize, self.x as usize));

        let width = self.input.dim().1 as i32;
        let height = self.input.dim().0 as i32;

        match self.dir {
            Direction::Right => match res {
                Some(_) => {
                    self.x += 1;
                    res
                }
                None => {
                    if self.y == height - 1 {
                        return None;
                    }
                    self.x = 0;
                    self.y += 1;
                    Some(&DELIMITER)
                }
            },

            Direction::Left => match res {
                Some(_) => {
                    self.x -= 1;
                    res
                }
                None => {
                    if self.y == height - 1 {
                        return None;
                    }
                    self.x = width - 1;
                    self.y += 1;
                    Some(&DELIMITER)
                }
            },

            Direction::Up => match res {
                Some(_) => {
                    self.y -= 1;
                    res
                }
                None => {
                    if self.x == width - 1 {
                        return None;
                    }
                    self.y = height - 1;
                    self.x += 1;
                    Some(&DELIMITER)
                }
            },

            Direction::Down => match res {
                Some(_) => {
                    self.y += 1;
                    res
                }
                None => {
                    if self.x == width - 1 {
                        return None;
                    }
                    self.y = 0;
                    self.x += 1;
                    Some(&DELIMITER)
                }
            },

            Direction::DiagUpRight => match res {
                Some(_) => {
                    self.y -= 1;
                    self.x += 1;
                    res
                }
                None => {
                    if self.x == width && self.y == height - 2 {
                        return None;
                    }
                    if self.x == width {
                        self.x = self.y + 2;
                        self.y = height - 1;
                    } else {
                        self.y = self.x;
                        self.x = 0;
                    }
                    Some(&DELIMITER)
                }
            },

            Direction::DiagUpLeft => match res {
                Some(_) => {
                    self.y -= 1;
                    self.x -= 1;
                    res
                }
                None => {
                    if self.x == width - 2 && self.y == -1 {
                        return None;
                    }
                    if self.y == -1 {
                        self.y = width - 1 - (self.x + 1) - 1;
                        self.x = height - 1;
                    } else {
                        self.x = height - 1 - self.y;
                        self.y = height - 1;
                    }
                    Some(&DELIMITER)
                }
            },

            Direction::DiagDownRight => match res {
                Some(_) => {
                    self.y += 1;
                    self.x += 1;
                    res
                }
                None => {
                    if self.x == width && self.y == 1 {
                        return None;
                    }
                    if self.x == width {
                        self.x = height - 1 - (self.y - 1) + 1;
                        self.y = 0;
                    } else {
                        self.y = width - 1 - self.x;
                        self.x = 0;
                    }
                    Some(&DELIMITER)
                }
            },

            Direction::DiagDownLeft => match res {
                Some(_) => {
                    self.y += 1;
                    self.x -= 1;
                    res
                }
                None => {
                    if self.x == width - 2 && self.y == height {
                        return None;
                    }
                    if self.y == height {
                        self.y = self.x + 2;
                        self.x = width - 1;
                    } else {
                        self.x = self.y;
                        self.y = 0;
                    }
                    Some(&DELIMITER)
                }
            },
        }
    }
}

fn parse_input(input: &str) -> Result<Array2<char>> {
    let size = input.split('\n').next().ok_or(anyhow!(""))?.len();
    let arr = Array::from_iter(input.chars().filter(|x| *x != '\n'));
    let arr_len = arr.len();
    let arr = arr
        .into_shape_clone((size, size))
        .with_context(|| format!("Bad shape : {} -> ({},{})", arr_len, &size, &size))?;
    Ok(arr)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input).unwrap();
    let re = Regex::new(r"XMAS").ok()?;

    let mut res: u32 = 0;

    for dir in Direction::iter() {
        let expl = Explorer::new(dir, &input).ok()?;
        let expl_str = Box::<str>::from_iter(expl);
        res += re.find_iter(&expl_str).count() as u32;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input).unwrap();
    let mut res: u32 = 0;
    let dirs = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];

    for x in 0..(input.dim().1 as isize) {
        for y in 0..(input.dim().0 as isize) {
            if let Some('A') = input.get((y as usize, x as usize)) {
                res += dirs
                    .map(|(fst, snd)| {
                        match (
                            input.get(((y + fst.1) as usize, (x + fst.0) as usize)),
                            input.get(((y + snd.1) as usize, (x + snd.0) as usize)),
                        ) {
                            (Some('M'), Some('S')) | (Some('S'), Some('M')) => 1,
                            _ => 0,
                        }
                    })
                    .iter()
                    .sum::<u32>() >> 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
