use std::{collections::VecDeque, fmt::Display, ops::AddAssign};

use anyhow::{anyhow, Error, Result};
use ndarray::Array2;

advent_of_code::solution!(15);

enum Cell {
    Wall,
    Box,
    Robot,
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Box => write!(f, "O"),
            Cell::Robot => write!(f, "@"),
            Cell::Empty => write!(f, "."),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Wall),
            'O' => Ok(Cell::Box),
            '@' => Ok(Cell::Robot),
            '.' => Ok(Cell::Empty),
            c => Err(anyhow!("Failed to parse cell \"{}\"", c)),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Up => write!(f, "^"),
            Instruction::Right => write!(f, ">"),
            Instruction::Down => write!(f, "v"),
            Instruction::Left => write!(f, "<"),
        }
    }
}

impl TryFrom<char> for Instruction {
    type Error = Error;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            _ => Err(anyhow!("Failed to parse instruction \"{}\"", value)),
        }
    }
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

/*
 * No checks are done on this function, use cautiously !
 */
impl AddAssign<&Instruction> for Position {
    fn add_assign(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Up => self.y -= 1,
            Instruction::Right => self.x += 1,
            Instruction::Down => self.y += 1,
            Instruction::Left => self.x -= 1,
        }
    }
}

struct Warehouse {
    arr: Array2<Cell>,
    instructions: VecDeque<Instruction>,
    robot: Position,
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // First display the array
        for row in self.arr.rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }

        // Then display instructions
        for instr in &self.instructions {
            write!(f, "{}", instr)?;
        }

        Ok(())
    }
}

impl Warehouse {
    fn parse(input: &str) -> Result<Self> {
        let height = input
            .lines()
            .count()
            .checked_sub(2)
            .ok_or(anyhow!("Not enough lines"))?;
        let width = input.lines().next().unwrap().len();
        let lines = input.lines();

        let arr = lines
            .clone()
            .take(height)
            .map(|l| {
                Ok(l.chars()
                    .map(|c| {
                        let cell: Cell = c.try_into()?;
                        Ok(cell)
                    })
                    .collect::<Result<Vec<Cell>>>()?)
            })
            .collect::<Result<Vec<Vec<Cell>>>>()?
            .into_iter()
            .flatten()
            .collect();
        let arr = Array2::from_shape_vec((height, width), arr)?;

        let instructions = lines
            .skip(height + 1)
            .next()
            .unwrap()
            .chars()
            .map(|c| Ok(c.try_into()?))
            .collect::<Result<VecDeque<Instruction>>>()?;

        let robot_pos = (|| {
            for y in 0..arr.dim().0 {
                for x in 0..arr.dim().1 {
                    let cell = arr.get((y, x)).unwrap();
                    if let Cell::Robot = cell {
                        return Ok(Position { x, y });
                    }
                }
            }
            Err(anyhow!("No robot found in initial map"))
        })()?;

        Ok(Self {
            arr,
            instructions,
            robot: robot_pos,
        })
    }

    /*
     * Returns an Option to a position representing a neighbor, if it exists.
     */
    fn get_neighbor(&self, pos: &Position, instr: &Instruction) -> Option<Position> {
        match instr {
            Instruction::Up => pos
                .y
                .checked_sub(1)
                .and_then(|new_y| Some(Position { x: pos.x, y: new_y })),
            Instruction::Right => {
                let new_x = pos.x + 1;
                if new_x < self.arr.dim().1 {
                    Some(Position { x: new_x, y: pos.y })
                } else {
                    None
                }
            }
            Instruction::Down => {
                let new_y = pos.y + 1;
                if new_y < self.arr.dim().0 {
                    Some(Position { x: pos.x, y: new_y })
                } else {
                    None
                }
            }
            Instruction::Left => pos
                .x
                .checked_sub(1)
                .and_then(|new_x| Some(Position { x: new_x, y: pos.y })),
        }
    }

    /*
     * Returns <true> if this cell was able to move
     */
    fn step_cell(&mut self, pos: &Position, instr: &Instruction) -> Result<bool> {
        match self.arr.get((pos.y, pos.x)).ok_or(anyhow!(
            "Failed to get cell at ({},{})",
            pos.x,
            pos.y
        ))? {
            Cell::Wall => Ok(false),
            // A box or the robot can move only iff the neighboring cell can also move
            Cell::Box | Cell::Robot => {
                let neighbor = self.get_neighbor(pos, instr);
                if let Some(neighbor) = neighbor {
                    if self.step_cell(&neighbor, instr)? {
                        // Swap and replace current by empty
                        self.arr.swap((pos.y, pos.x), (neighbor.y, neighbor.x));
                        let current = self.arr.get_mut((pos.y, pos.x)).unwrap();
                        *current = Cell::Empty;
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            Cell::Empty => Ok(true),
        }
    }

    /*
     * Step the entire map. Returns true if instruction was taken into account, false otherwise.
     */
    fn step(&mut self) -> Result<bool> {
        // If no more instructions, nothing moves
        if let Some(instr) = self.instructions.pop_front() {
            // Move robot position only iff the cell was effectively moved
            if self.step_cell(&self.robot.clone(), &instr)? {
                self.robot += &instr;
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn coordinates(&self) -> u64 {
        let mut res = 0;
        for y in 0..self.arr.dim().0 {
            for x in 0..self.arr.dim().1 {
                if let Some(Cell::Box) = self.arr.get((y, x)) {
                    res += (100 * y + x) as u64;
                }
            }
        }
        res
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut warehouse = Warehouse::parse(input).unwrap();
    println!("{}", warehouse);
    while warehouse.step().unwrap() {
        // println!("{}", warehouse);
    }
    Some(warehouse.coordinates())
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
