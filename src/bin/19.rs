use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use anyhow::{anyhow, Result};
use regex::Regex;

advent_of_code::solution!(19);

fn parse_regex(line: &str) -> Result<Regex> {
    let mut regex = "^(".to_owned();
    regex.extend(line.replace(", ", "|").chars());
    regex.push_str(")+$");
    Ok(Regex::new(&regex)?)
}

/*
 * A regex is just an NFA, which is exactly what we need here.
 */
pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let regex = parse_regex(lines.next().ok_or(anyhow!("Not enough lines !")).unwrap()).unwrap();
    lines.next();
    Some(lines.fold(0, |acc, l| acc + (regex.is_match(l) as u32)))
}

/*
 * Part two : for fun, I reimplemented an NFA to count
 * how many paths lead to a valid token
 * -> It works, but very inefficient :(
 */
#[derive(Debug)]
enum State {
    Start(RefCell<HashMap<char, Vec<Weak<State>>>>), // The `start` state S is also the final state
    Transition(HashMap<char, Rc<State>>), // A `transitional` state is every state except S
}

#[derive(Debug)]
#[allow(dead_code)]
struct NFA {
    start: Rc<State>,
    states: Vec<Rc<State>>,
}

impl NFA {
    fn parse(line: &str) -> Result<Self> {
        let start = State::Start(RefCell::new(HashMap::new()));
        let start = Rc::new(start);
        let mut states = vec![];

        for elem in line.split(", ") {
            // First create our new state chain
            let mut next_state = start.clone();
            for c in elem.chars().rev() {
                let mut next = HashMap::<char, Rc<State>>::new();
                next.insert(c, next_state);
                next_state = Rc::new(State::Transition(next));
            }

            // Add the second state to nfa : this is mandatory
            // to ensure that the Rc is not dropped to 0
            states.push(next_state.clone());

            // Then add it to S's successors
            if let State::Start(next) = start.as_ref() {
                let mut start_next = next.borrow_mut();
                let c = elem
                    .chars()
                    .next()
                    .ok_or(anyhow!("Failed to read a single char in {}", elem))?;

                let weakref = Rc::downgrade(&next_state);
                match start_next.get_mut(&c) {
                    Some(nexts) => {
                        nexts.push(weakref);
                    }
                    None => {
                        start_next.insert(c, vec![weakref]);
                    }
                };
            }
        }

        Ok(NFA { start, states })
    }

    /*
     * Recursive function to compute how many paths lead to a valid solution
     */
    fn _count(&self, design: &str, curr_state: &Rc<State>) -> Result<u32> {
        let len = design.len();
        let curr_state = curr_state.as_ref();

        // Stop condition : there are no more chars to read
        if len == 0 {
            // S is also the only final state, so increase by one
            // the result if its the current state
            if let State::Start(_) = curr_state {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }

        let c = design.chars().next().unwrap(); // Unwrap ok here because of previous condition
        match curr_state {
            // For S only, need to sum how many valid paths are available
            State::Start(next) => match next.borrow().get(&c) {
                Some(next_states) => next_states.iter().fold(Ok(0), |acc, state| match acc {
                    Ok(acc) => Ok(acc
                        + self._count(
                            &design,
                            &state
                                .upgrade()
                                .ok_or(anyhow!("Failed to upgrade weak pointer"))?,
                        )?),
                    Err(err) => Err(err),
                }),
                None => Ok(0),
            },
            // Else, consume one more char
            State::Transition(next) => match next.get(&c) {
                Some(next_state) => self._count(&design[1..], next_state),
                None => Ok(0),
            },
        }
    }

    /*
     * Returns the number of possible ways to come up with design <design>
     */
    fn count(&self, design: &str) -> Result<u32> {
        self._count(design, &self.start)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // let mut lines = input.lines();
    // let nfa = NFA::parse(lines.next().ok_or(anyhow!("Not enough lines !")).unwrap()).unwrap();
    // lines.next();
    // Some(lines.fold(0, |acc, l| acc + nfa.count(l).unwrap()))
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
