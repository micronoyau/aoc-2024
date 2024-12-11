use anyhow::{anyhow, Result};
use std::fmt::Display;

advent_of_code::solution!(9);

#[derive(Clone)]
enum Chunk {
    FULL(u64),
    EMPTY,
}

// For p1 only
struct DiskMap {
    chunks: Vec<Chunk>,
}

impl DiskMap {
    fn new() -> Self {
        DiskMap { chunks: vec![] }
    }

    fn parse(input: &str) -> Result<Self> {
        let mut disk = DiskMap::new();

        let mut count = 0;
        let parse_digit = |x: char| -> Result<u8> {
            let x: u8 = x
                .to_digit(10)
                .ok_or(anyhow!("Failed to parse digit : \"{}\"", x))? as u8;
            Ok(x)
        };

        let mut input_iter = input.chars();
        while let Some(chunk_size) = input_iter.next() {
            let chunk_size = parse_digit(chunk_size)?;
            // Full
            if count % 2 == 0 {
                disk.chunks.extend(
                    [Chunk::FULL(count >> 1)]
                        .into_iter()
                        .cycle()
                        .take(chunk_size as usize),
                );
            }
            // Empty
            else {
                disk.chunks
                    .extend([Chunk::EMPTY].into_iter().cycle().take(chunk_size as usize));
            }
            count += 1;
        }

        Ok(disk)
    }

    fn compress(&mut self) -> Result<()> {
        // Take empty and full indices
        let empty_chunks = self
            .chunks
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                Chunk::EMPTY => Some(i),
                Chunk::FULL(_) => None,
            })
            .collect::<Vec<usize>>();
        let full_chunks = self
            .chunks
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                Chunk::FULL(_) => Some(i),
                Chunk::EMPTY => None,
            })
            .rev()
            .collect::<Vec<usize>>();

        let mut full_chunks_iter = full_chunks.iter();
        let mut empty_chunks_iter = empty_chunks.iter();
        while let (Some(full), Some(empty)) = (full_chunks_iter.next(), empty_chunks_iter.next()) {
            if full < empty {
                break;
            }
            self.chunks.swap(*full, *empty);
        }

        Ok(())
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.chunks.iter() {
            match chunk {
                Chunk::FULL(c) => write!(f, "{}", c),
                Chunk::EMPTY => write!(f, "."),
            }?;
        }
        Ok(())
    }
}

// For p2 only
// Could be an actual file, or a free space
struct Space {
    size: u8,
    index: usize,
    chunk_type: Chunk,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr: String = [match self.chunk_type {
            Chunk::FULL(uid) => uid.to_string(),
            Chunk::EMPTY => ".".to_owned(),
        }]
        .into_iter()
        .cycle()
        .take(self.size as usize)
        .collect();
        write!(f, "{}", repr)
    }
}

// For p2 only
struct SpaceDiskMap {
    free: Vec<Space>,
    used: Vec<Space>,
}

// Buggy
impl Display for SpaceDiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let free: Vec<&Space> = self.free.iter().collect();
        let mut spaces: Vec<&Space> = self.used.iter().collect();
        spaces.extend(free);
        spaces.sort_by(|x, y| x.index.cmp(&y.index));
        for s in spaces {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl SpaceDiskMap {
    fn new() -> Self {
        SpaceDiskMap {
            free: vec![],
            used: vec![],
        }
    }

    fn parse(input: &str) -> Result<SpaceDiskMap> {
        let mut disk = SpaceDiskMap::new();

        let mut count = 0;
        let parse_digit = |x: char| -> Result<u8> {
            let x: u8 = x
                .to_digit(10)
                .ok_or(anyhow!("Failed to parse digit : \"{}\"", x))? as u8;
            Ok(x)
        };

        let mut input_iter = input.chars();
        let mut index: usize = 0;
        while let Some(size) = input_iter.next() {
            let size = parse_digit(size)?;
            // Full
            if count % 2 == 0 {
                disk.used.push(Space {
                    size,
                    index,
                    chunk_type: Chunk::FULL(count >> 1),
                });
            }
            // Empty
            else {
                disk.free.push(Space {
                    size,
                    index,
                    chunk_type: Chunk::EMPTY,
                });
            }
            count += 1;
            index += size as usize;
        }

        Ok(disk)
    }

    fn compress(&mut self) -> Result<()> {
        for used in self.used.iter_mut().rev() {
            if let Some(free) = self
                .free
                .iter_mut()
                .filter(|f| f.index < used.index && f.size >= used.size)
                .next()
            {
                used.index = free.index;
                free.size -= used.size;
                free.index += used.size as usize;
            }
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = DiskMap::parse(input).unwrap();
    disk.compress().unwrap();

    let mut chunks_iter = disk.chunks.iter().enumerate();
    let mut res = 0;
    while let Some((i, Chunk::FULL(val))) = chunks_iter.next() {
        res += i * (*val as usize);
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = SpaceDiskMap::parse(input).unwrap();
    disk.compress().unwrap();

    let mut res = 0;
    for space in disk.used {
        let index = space.index as u64;
        let size = space.size as u64;
        let a: Result<u64> = match space.chunk_type {
            Chunk::FULL(uid) => Ok(uid * (index..(index + size)).sum::<u64>()),
            Chunk::EMPTY => Err(anyhow!("Free space in used list... aborting")),
        };
        res += a.unwrap();
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
