advent_of_code::solution!(16);

use std::{collections::HashSet, fmt::Display};

use anyhow::{anyhow, Error, Result};
use ndarray::Array2;

const WALL_CHR: char = '#';
const EMPTY_CHR: char = '.';
const START_CHR: char = 'S';
const END_CHR: char = 'E';

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
        }
    }
}

/*
 * A structure representing the state at some point during maze traversal.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: Position,
    dir: Direction,
}

/*
 * Structure representing a possible position taken at some point,
 * <next> is the next priority node to explore
 * Should be kept sorted in increasing order :
 * node->cost <= node->next->cost
 */
#[derive(Debug, PartialEq, Eq)]
struct Node {
    state: State,
    next: Option<Box<Node>>,
    cost: u64,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({},{}) : c={} | ",
            self.state.dir, self.state.pos.x, self.state.pos.y, self.cost
        )?;
        if let Some(node) = &self.next {
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

/*
 * Structures related to map objects
 */
#[derive(Debug)]
enum CellType {
    Wall,
    Empty,
    Start,
    End,
}

impl TryFrom<char> for CellType {
    type Error = Error;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            WALL_CHR => Ok(Self::Wall),
            EMPTY_CHR => Ok(Self::Empty),
            START_CHR => Ok(Self::Start),
            END_CHR => Ok(Self::End),
            c => Err(anyhow!("Failed to parse character {} for cell type", c)),
        }
    }
}

#[derive(Debug)]
struct Cell {
    pos: Position,
    typ: CellType,
}

#[derive(Debug)]
struct Map {
    cells: Array2<Cell>,
    states: HashSet<State>,
}

impl Map {
    fn parse(input: &str) -> Result<(Map, Node)> {
        let mut cells = vec![];
        let mut node = Node {
            state: State {
                pos: Position { x: 0, y: 0 },
                dir: Direction::Right,
            },
            next: None,
            cost: 0,
        };
        let height = input.lines().count();
        let width = input
            .lines()
            .next()
            .ok_or(anyhow!("Input does not even have one line !"))?
            .chars()
            .count();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                cells.push(Cell {
                    pos: Position { x, y },
                    typ: c.try_into()?,
                });
                if c == START_CHR {
                    node.state.pos = Position { x, y };
                }
            }
        }
        let cells = Array2::from_shape_vec((height, width), cells)?;
        Ok((
            Map {
                cells,
                states: HashSet::new(),
            },
            node,
        ))
    }

    /*
     * Get neighbor directly in front of the node
     */
    fn get_front_neighbor(&self, node: &Node) -> Option<Node> {
        let front_neighbor = match node.state.dir {
            Direction::Up => self
                .cells
                .get((node.state.pos.y.checked_sub(1)?, node.state.pos.x)),
            Direction::Right => self.cells.get((node.state.pos.y, node.state.pos.x + 1)),
            Direction::Down => self.cells.get((node.state.pos.y + 1, node.state.pos.x)),
            Direction::Left => self
                .cells
                .get((node.state.pos.y, node.state.pos.x.checked_sub(1)?)),
        }?;
        match front_neighbor.typ {
            CellType::Empty | CellType::End => Some(Node {
                state: State {
                    pos: front_neighbor.pos,
                    dir: node.state.dir,
                },
                next: None,
                cost: node.cost + 1,
            }),
            CellType::Wall | CellType::Start => None,
        }
    }

    fn _get_side_neighbors(&self, node: &Node) -> Option<Vec<Node>> {
        let side_neighbors = match node.state.dir {
            Direction::Up | Direction::Down => [
                (
                    self.cells
                        .get((node.state.pos.y, node.state.pos.x.checked_sub(1)?))?,
                    Direction::Left,
                ),
                (
                    self.cells.get((node.state.pos.y, node.state.pos.x + 1))?,
                    Direction::Right,
                ),
            ],
            Direction::Right | Direction::Left => [
                (
                    self.cells
                        .get((node.state.pos.y.checked_sub(1)?, node.state.pos.x))?,
                    Direction::Up,
                ),
                (
                    self.cells.get((node.state.pos.y + 1, node.state.pos.x))?,
                    Direction::Down,
                ),
            ],
        };

        Some(
            side_neighbors
                .into_iter()
                // Add rotating neighbors
                .filter_map(|(neighbor, dir)| match neighbor.typ {
                    CellType::Empty | CellType::End => Some(Node {
                        state: State {
                            pos: neighbor.pos,
                            dir,
                        },
                        next: None,
                        cost: node.cost + 1001, // + 1000 to turn, +1 to step
                    }),
                    CellType::Wall | CellType::Start => None,
                })
                .collect(),
        )
    }

    /*
     * Get neighbor on the sides of the node
     */
    fn get_side_neighbors(&self, node: &Node) -> Vec<Node> {
        self._get_side_neighbors(node).unwrap_or(vec![])
    }

    /*
     * Returns all neighbors, sorted by increasing cost.
     * Skips neighbors already explored or to be explored.
     */
    fn get_neighbors(&self, node: &Node) -> Option<Box<Node>> {
        let mut neighbors = self.get_side_neighbors(node);
        if let Some(front) = self.get_front_neighbor(node) {
            neighbors.push(front);
        }
        // Remove already explored / to be explored states
        let mut neighbors: Vec<Node> = neighbors
            .into_iter()
            .filter(|n| !self.states.contains(&n.state))
            .collect();
        // Sort
        neighbors.sort();
        neighbors.reverse(); // Reverse because we will use pop

        // Then format it as a linked list
        let mut neighbors_ll: Option<Box<Node>> = None;
        let mut current_opt = &mut neighbors_ll;
        while let Some(current) = neighbors.pop() {
            *current_opt = Some(Box::new(current));
            current_opt = &mut current_opt.as_mut().unwrap().next;
        }

        neighbors_ll
    }

    /*
     * Perform one step and return the new node to explore
     */
    fn step(&mut self, node: Option<Box<Node>>) -> Option<Box<Node>> {
        let node = node?;
        let neighbors = self.get_neighbors(&node);
        // Add neighbors states to explored/to be explored states
        let mut curr_neighbor = &neighbors;
        while let Some(neighbor) = curr_neighbor {
            self.states.insert(neighbor.state);
            curr_neighbor = &neighbor.next;
        }

        let mut merged: Option<Box<Node>> = None;
        let mut next_node_pos = &mut merged;
        let mut node_opt = node.next;
        let mut neighbor_opt = neighbors;
        loop {
            let mut node = match node_opt {
                Some(node) => node,
                None => {
                    *next_node_pos = neighbor_opt;
                    break;
                }
            };
            let mut neighbor = match neighbor_opt {
                Some(neighbor) => neighbor,
                None => {
                    *next_node_pos = Some(node);
                    break;
                }
            };

            if neighbor.cost < node.cost {
                neighbor_opt = neighbor.next.take();
                node_opt = Some(node);
                *next_node_pos = Some(neighbor);
            } else {
                neighbor_opt = Some(neighbor);
                node_opt = node.next.take();
                *next_node_pos = Some(node);
            }

            next_node_pos = &mut next_node_pos.as_mut().unwrap().next;
        }

        merged
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut map, node) = Map::parse(input).unwrap();
    let mut node = Some(Box::new(node));

    loop {
        node = map.step(node);
        // Stop when reached end, or if no more nodes to explore
        match &node {
            None => {
                break;
            }
            Some(node) => {
                // println!("{}", node);
                let cell = map.cells.get((node.state.pos.y, node.state.pos.x)).unwrap();
                if let CellType::End = cell.typ {
                    return Some(node.cost);
                }
            }
        }
    }

    None
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
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
