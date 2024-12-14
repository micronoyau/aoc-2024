use anyhow::{anyhow, Result};
use ndarray::{Array, Array2};
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};
use strum::{EnumIter, IntoEnumIterator};

advent_of_code::solution!(12);

const EXPLORED: char = '*';

fn parse(input: &str) -> Result<Array2<char>> {
    let width = input
        .chars()
        .fold((0, true), |acc, elem| {
            if acc.1 {
                let valid = elem != '\n';
                return (acc.0 + (valid as usize), valid);
            }
            acc
        })
        .0;
    let height = input.lines().count();
    let chars: Vec<char> = input.chars().filter(|x| *x != '\n').collect();
    Array::from_shape_vec((height, width), chars).map_err(|err| {
        anyhow!(
            "Failed to build array from dims {}x{}: {}",
            height,
            width,
            err
        )
    })
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Add<&Direction> for &Position {
    type Output = Option<Position>;
    fn add(self, rhs: &Direction) -> Self::Output {
        match rhs {
            Direction::Up => Some(Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            }),
            Direction::Right => Some(Position {
                x: self.x.checked_add(1)?,
                y: self.y,
            }),
            Direction::Down => Some(Position {
                x: self.x,
                y: self.y.checked_add(1)?,
            }),
            Direction::Left => Some(Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            }),
        }
    }
}

/*
 * Explore and compute area related to starting position <pos>.
 */
fn explore(
    arr: &mut Array2<char>,
    arr_original: &Array2<char>,
    expl_stack: &mut VecDeque<Position>,
    pos: Position,
    c: char,
) -> u64 {
    let mut area = 0;
    let mut perimeter = 0;
    let mut siblings_expl_stack = VecDeque::from([pos]);
    let mut arr_clone = arr_original.clone();

    while let Some(pos) = siblings_expl_stack.pop_front() {
        if let (Some(plot), Some(plot_clone)) = (
            arr.get_mut((pos.y, pos.x)),
            arr_clone.get_mut((pos.y, pos.x)),
        ) {
            // If this position hash already been explored for this character, skip
            if *plot_clone == EXPLORED {
                continue;
            }
            // If this plot is a sibling
            else if *plot_clone == c {
                area += 1;
                let neighbors: Vec<Position> =
                    Direction::iter().filter_map(|dir| &pos + &dir).collect();
                perimeter += (4 - neighbors.len()) as u64; // Dont forget to add fences outside bounds
                siblings_expl_stack.extend(neighbors); // Explore neighbors
                *plot = EXPLORED; // Mark it as explored
                *plot_clone = EXPLORED;
            }
            // If this is a different plot
            else {
                perimeter += 1;
                // Add it to the original explored stack if not explored already
                if *plot != EXPLORED {
                    expl_stack.extend(vec![pos]);
                }
            }
        }
        // If outside of bounds, it means fence
        else {
            perimeter += 1;
        }
    }

    area * perimeter
}

/*
 * Compute total cost of a region
 */
fn compute_price(mut arr: Array2<char>) -> u64 {
    let mut res = 0;
    let arr_clone = arr.clone();
    let mut expl_stack = VecDeque::from([Position { x: 0, y: 0 }]);

    while let Some(pos) = expl_stack.pop_front() {
        if let Some(&c) = arr.get((pos.y, pos.x)) {
            // If never explored, start a new exploration
            if c != EXPLORED {
                res += explore(&mut arr, &arr_clone, &mut expl_stack, pos, c);
            }
        }
    }

    res
}

/*
 ******** Part 2 only ********
 */

#[derive(Debug)]
struct Plant {
    pos: Position,
    discounts: HashSet<Direction>,
}

#[derive(Debug)]
struct Fence {
    dir: Direction,
    pos: Option<Position>,
}

#[derive(Debug)]
enum Neighbor {
    Sibling(Position),
    Fence(Fence),
}

impl Plant {
    /*
     * Return uncharted neighbors of plant in 2 categories : siblings or fence
     */
    fn neighbors(&self, arr: &Array2<char>, c: char) -> Vec<Neighbor> {
        Direction::iter()
            .filter_map(|dir| {
                if let Some(pos) = &self.pos + &dir {
                    if let Some(&cn) = arr.get((pos.y, pos.x)) {
                        if c == cn {
                            return Some(Neighbor::Sibling(pos));
                        }
                        // Dont account for already explored plants
                        else if cn == EXPLORED {
                            return None;
                        }
                        return Some(Neighbor::Fence(Fence {
                            dir,
                            pos: Some(pos),
                        }));
                    }
                }
                Some(Neighbor::Fence(Fence { dir, pos: None }))
            })
            .collect()
    }
}


/*
 * Explore and compute area related to starting position <pos>.
 */
fn explore_discount(
    arr: &mut Array2<char>,
    original_arr: &Array2<char>,
    pos: Position,
    queue: &mut VecDeque<Position>,
    c: char,
) -> Result<u64> {
    let mut area = 0;
    let mut perimeter = 0;
    // Array to be edited during exploration
    let mut local_arr = original_arr.clone();
    let plant = Plant {
        pos,
        discounts: HashSet::new(),
    };
    // The queue for contiguous plants of the same type
    let mut local_queue = VecDeque::from([plant]);

    while let Some(plant) = local_queue.pop_front() {
        // First compute neighbor discounts and compute current perimeter
        let neighbors = plant.neighbors(&local_arr, c);
        let mut neighbor_discounts = HashSet::<Direction>::new();
        for n in &neighbors {
            if let Neighbor::Fence(fence) = n {
                // Add direction in neighbor's discount pack
                neighbor_discounts.insert(fence.dir);
                // Increase perimeter iff no discount
                if !plant.discounts.contains(&fence.dir) {
                    perimeter += 1;
                }
                // If uncharted yet, add it to queue
                if let Some(pos) = fence.pos {
                    if let Some(&c) = arr.get((pos.y, pos.x)) {
                        if c != EXPLORED {
                            if !queue.contains(&pos) {
                                queue.push_back(pos);
                            }
                        }
                    }
                }
            }
        }

        // Then add neighbors to queue and propagate discount
        for n in neighbors {
            if let Neighbor::Sibling(pos) = n {
                // Check if this neighbor is already present in queue
                let plant_queue = local_queue.iter_mut().fold(None, |acc, plant| {
                    acc.or(if plant.pos == pos {
                        return Some(plant);
                    } else {
                        None
                    })
                });
                match plant_queue {
                    // If so, just update the discounts
                    Some(plant_queue) => {
                        plant_queue.discounts.extend(neighbor_discounts.clone());
                    }
                    // Else, create it
                    None => {
                        local_queue.push_back({
                            Plant {
                                pos,
                                discounts: neighbor_discounts.clone(),
                            }
                        });
                    }
                }
            }
        }

        // Update arrays
        let c = local_arr
            .get_mut((plant.pos.y, plant.pos.x))
            .ok_or(anyhow!(
                "Failed to update plant type at ({},{})",
                plant.pos.x,
                plant.pos.y
            ))?;
        *c = EXPLORED;
        let c = arr.get_mut((plant.pos.y, plant.pos.x)).unwrap();
        *c = EXPLORED;

        area += 1;
    }

    Ok(area * perimeter)
}

/*
 * Compute total price with discount applied
 */
fn compute_price_with_discount(mut arr: Array2<char>) -> Result<u64> {
    let original_arr = arr.clone();
    let mut cost = 0;
    let mut queue = VecDeque::from([Position { x: 0, y: 0 }]);

    while let Some(pos) = queue.pop_front() {
        // This position might have been explored meanwhile
        let c = *arr.get((pos.y, pos.x)).ok_or(anyhow!(
            "Failed to read array at ({},{})",
            pos.x,
            pos.y
        ))?;
        if c != EXPLORED {
            cost += explore_discount(&mut arr, &original_arr, pos, &mut queue, c)?;
        }
    }

    Ok(cost)
}

pub fn part_one(input: &str) -> Option<u64> {
    let arr = parse(input).unwrap();
    Some(compute_price(arr))
}

pub fn part_two(input: &str) -> Option<u64> {
    let arr = parse(input).unwrap();
    Some(compute_price_with_discount(arr).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}