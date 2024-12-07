use regex::{Match, Regex};

advent_of_code::solution!(3);

fn regex_mul() -> Option<Regex> {
    Regex::new(r"mul\((\d*),(\d*)\)").ok()
}

fn regex_do() -> Option<Regex> {
    Regex::new(r"do\(\)").ok()
}

fn regex_dont() -> Option<Regex> {
    Regex::new(r"don't\(\)").ok()
}

/*
 * A state is represented by 5 pointers :
 * - a pointer for the closest `do` instruction BEFORE the current mul
 * - a pointer for the closest `do` instruction AFTER the current mul
 * - a pointer for the closest `dont` instruction BEFORE the current mul
 * - a pointer for the closest `dont` instruction AFTER the current mul
 * - the current `mul` instruction
 */
struct State<'a> {
    do_before_ptr: Option<Match<'a>>,
    do_after_ptr: Option<Match<'a>>,
    dont_before_ptr: Option<Match<'a>>,
    dont_after_ptr: Option<Match<'a>>,
    mul_ptr: usize,
}

/*
 * Update before and after ptrs from an iterator
 */
fn update_ptrs<'a, I>(
    iter: &mut I,
    before_ptr: Option<Match<'a>>,
    after_ptr: Option<Match<'a>>,
    mul_ptr: usize,
) -> (Option<Match<'a>>, Option<Match<'a>>)
where
    I: Iterator<Item = Match<'a>>,
{
    let mut new_before_ptr = before_ptr;
    let mut new_after_ptr = after_ptr;

    while let Some(new_after_ptr_val) = new_after_ptr {
        if new_after_ptr_val.start() < mul_ptr {
            new_before_ptr = new_after_ptr;
            new_after_ptr = iter.next();
        } else {
            break;
        }
    }

    (new_before_ptr, new_after_ptr)
}

/*
 * Update state
 */
fn update_state<'a, I>(dos: &mut I, donts: &mut I, state: &mut State<'a>)
where
    I: Iterator<Item = Match<'a>>,
{
    (state.do_before_ptr, state.do_after_ptr) =
        update_ptrs(dos, state.do_before_ptr, state.do_after_ptr, state.mul_ptr);
    (state.dont_before_ptr, state.dont_after_ptr) = update_ptrs(
        donts,
        state.dont_before_ptr,
        state.dont_after_ptr,
        state.mul_ptr,
    );
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ret: u32 = 0;
    let re = regex_mul()?;
    for capture in re.captures_iter(input) {
        let l = capture.get(1)?.as_str().parse::<u32>().ok()?;
        let r = capture.get(2)?.as_str().parse::<u32>().ok()?;
        ret += l * r;
    }
    Some(ret)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ret: u32 = 0;
    let re_mul = regex_mul()?;

    let r_dos = regex_do()?;
    let r_donts = regex_dont()?;
    let mut dos = r_dos.find_iter(input);
    let mut donts = r_donts.find_iter(input);

    let mut state = State {
        do_before_ptr: None,
        do_after_ptr: dos.next(),
        dont_before_ptr: None,
        dont_after_ptr: donts.next(),
        mul_ptr: 0,
    };
    let mut enabled = true;

    for capture in re_mul.captures_iter(input) {
        let l = capture.get(1)?.as_str().parse::<u32>().ok()?;
        let r = capture.get(2)?.as_str().parse::<u32>().ok()?;
        state.mul_ptr = capture.get(0)?.start();

        // First update state
        update_state(&mut dos, &mut donts, &mut state);

        match (state.do_before_ptr, state.dont_before_ptr) {
            (Some(do_ptr), Some(dont_ptr)) => {
                if enabled && do_ptr.start() < dont_ptr.start() {
                    enabled = false;
                } else if !enabled && dont_ptr.start() < do_ptr.start() {
                    enabled = true;
                }
            }
            (Some(_), None) => {
                enabled = true;
            }
            (None, Some(_)) => {
                enabled = false;
            }
            _ => {}
        }

        // Then add or not
        if enabled {
            ret += l * r;
        }
    }

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
