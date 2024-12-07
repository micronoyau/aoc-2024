use std::num::ParseIntError;

advent_of_code::solution!(2);

fn parse(input: &str) -> Option<Vec<Vec<u32>>> {
    let mut reports: Vec<Vec<u32>> = vec![];
    for l in input.lines() {
        reports.push(
            l.split_whitespace()
                .map(|x| x.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()
                .ok()?,
        );
    }
    Some(reports)
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse(input)?;
    let safe_reports = reports.iter().filter(|x| {
        let mut x_iter = x.into_iter().map(|x| *x as i64);
        // If increasing or decreasing, then the sign of the difference should remain constant
        match (x_iter.next(), x_iter.next()) {
            (Some(fst), Some(snd)) => {
                let fst_diff = snd - fst;
                return (1 <= fst_diff.abs())
                    && (fst_diff.abs() <= 3)
                    && x_iter
                        // (previous passed, previous difference, previous item value)
                        .fold((true, fst_diff, snd), |acc, elem| {
                            let diff: i64 = (elem as i64) - (acc.2 as i64);
                            (
                                acc.0
                                    && (diff * acc.1 > 0)
                                    && (1 <= diff.abs())
                                    && (diff.abs() <= 3),
                                diff,
                                elem,
                            )
                        })
                        .0;
            }
            _ => false,
        }
    });
    Some(safe_reports.count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse(input)?;
    let safe_reports = reports.iter().filter(|x| {
        let mut x_iter = x.into_iter().map(|x| *x as i64);
        // If increasing or decreasing, then the sign of the difference should remain constant
        match (x_iter.next(), x_iter.next()) {
            (Some(fst), Some(snd)) => {
                let fst_diff = snd - fst;
                let fst_cond = (1 <= fst_diff.abs()) && (fst_diff.abs() <= 3);
                let fst_skipped = !fst_cond;
                return x_iter
                    // (previous passed, previous difference, item index n-1, item index n-2,
                    // skipped)
                    .fold((true, fst_diff, snd, fst, fst_skipped), |acc, elem| {
                        let (prev_pass, prev_diff, prev_item, prev_prev_item, skipped) = acc;
                        let diff: i64 = elem - prev_item;
                        let cond = (diff * prev_diff > 0) && (1 <= diff.abs()) && (diff.abs() <= 3);
                        if !cond && !skipped {
                            return (prev_pass, prev_diff, prev_item, prev_prev_item, true);
                        }
                        (prev_pass && cond, diff, elem, prev_item, skipped)
                    })
                    .0;
            }
            _ => false,
        }
    });
    let a: Vec<&Vec<u32>> = safe_reports.collect();
    for b in a {
        println!("{:?}", b);
    }
    None
    // Some(safe_reports.count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
