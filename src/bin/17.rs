use std::{
    fmt::{Display, Formatter},
    iter::zip,
};

use anyhow::{anyhow, Error, Result};
use regex::Regex;

advent_of_code::solution!(17);

struct LiteralOperand {
    val: u64,
}

impl Display for LiteralOperand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl TryFrom<u64> for LiteralOperand {
    type Error = Error;
    fn try_from(val: u64) -> std::result::Result<Self, Self::Error> {
        Ok(Self { val })
    }
}

enum ComboOperand {
    Literal(u64),
    RegA,
    RegB,
    RegC,
}

impl TryFrom<u64> for ComboOperand {
    type Error = Error;
    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        if value <= 3 {
            Ok(Self::Literal(value))
        } else if value == 4 {
            Ok(Self::RegA)
        } else if value == 5 {
            Ok(Self::RegB)
        } else if value == 6 {
            Ok(Self::RegC)
        } else {
            Err(anyhow!("Invalid operand : {}", value))
        }
    }
}

impl Display for ComboOperand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ComboOperand::Literal(val) => write!(f, "{}", val),
            ComboOperand::RegA => write!(f, "regA"),
            ComboOperand::RegB => write!(f, "regB"),
            ComboOperand::RegC => write!(f, "regC"),
        }
    }
}

impl ComboOperand {
    fn get_value(&self, machine: &Machine) -> u64 {
        match self {
            ComboOperand::Literal(val) => *val,
            ComboOperand::RegA => machine.reg_a,
            ComboOperand::RegB => machine.reg_b,
            ComboOperand::RegC => machine.reg_c,
        }
    }
}

enum Opcode {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc(ComboOperand),
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl TryFrom<(u64, u64)> for Opcode {
    type Error = Error;
    fn try_from(value: (u64, u64)) -> std::result::Result<Self, Self::Error> {
        let opcode = value.0;
        let operand = value.1;
        match opcode {
            0 => Ok(Opcode::Adv(operand.try_into()?)),
            1 => Ok(Opcode::Bxl(operand.try_into()?)),
            2 => Ok(Opcode::Bst(operand.try_into()?)),
            3 => Ok(Opcode::Jnz(operand.try_into()?)),
            4 => Ok(Opcode::Bxc(operand.try_into()?)),
            5 => Ok(Opcode::Out(operand.try_into()?)),
            6 => Ok(Opcode::Bdv(operand.try_into()?)),
            7 => Ok(Opcode::Cdv(operand.try_into()?)),
            _ => Err(anyhow!("Invalid opcode {}", opcode)),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Opcode::Adv(operand) => write!(f, "adv {}", operand),
            Opcode::Bxl(operand) => write!(f, "bxl {}", operand),
            Opcode::Bst(operand) => write!(f, "bst {}", operand),
            Opcode::Jnz(operand) => write!(f, "jnz {}", operand),
            Opcode::Bxc(operand) => write!(f, "bxc {}", operand),
            Opcode::Out(operand) => write!(f, "out {}", operand),
            Opcode::Bdv(operand) => write!(f, "bdv {}", operand),
            Opcode::Cdv(operand) => write!(f, "cdv {}", operand),
        }
    }
}

struct Program {
    instrs: Vec<Opcode>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for instr in &self.instrs {
            writeln!(f, "{}", instr)?;
        }
        Ok(())
    }
}

impl Program {
    fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Self> {
        let mut instrs = vec![];
        let l = lines
            .next()
            .ok_or(anyhow!("Not enough lines in input file !"))?;
        let instrs_str = Regex::new(r"Program: ((\d\,)+\d)")?
            .captures(l)
            .and_then(|m| Some(m.get(1)?.as_str()))
            .ok_or(anyhow!("Failed to parse line {}", l))?;

        for (opcode, operand) in zip(
            instrs_str.chars().step_by(2),
            instrs_str.chars().step_by(2).skip(1),
        ) {
            // Assuming this has worked thanks to previous regex
            let opcode = opcode.to_digit(10).unwrap() as u64;
            let operand = operand.to_digit(10).unwrap() as u64;
            instrs.push((opcode, operand).try_into()?);
        }

        Ok(Self { instrs })
    }

    fn parse2<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<(String, Self)> {
        let mut instrs = vec![];
        let l = lines
            .next()
            .ok_or(anyhow!("Not enough lines in input file !"))?;
        let instrs_str = Regex::new(r"Program: ((\d\,)+\d)")?
            .captures(l)
            .and_then(|m| Some(m.get(1)?.as_str()))
            .ok_or(anyhow!("Failed to parse line {}", l))?;

        for (opcode, operand) in zip(
            instrs_str.chars().step_by(2),
            instrs_str.chars().step_by(2).skip(1),
        ) {
            // Assuming this has worked thanks to previous regex
            let opcode = opcode.to_digit(10).unwrap() as u64;
            let operand = operand.to_digit(10).unwrap() as u64;
            instrs.push((opcode, operand).try_into()?);
        }

        Ok((instrs_str.to_owned(), Self { instrs }))
    }
}

#[derive(Clone, Debug)]
struct Machine {
    ip: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

impl Machine {
    fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Self> {
        let err = || anyhow!("Not enough lines in input file !");
        let err2 = |l| anyhow!("Failed to parse line {}", l);
        let l = lines.next().ok_or(err())?;
        let reg_a = Regex::new(r"Register A: (\d+)")?
            .captures(l)
            .and_then(|m| Some(m.get(1)?.as_str()))
            .ok_or(err2(l))?
            .parse()?;
        let l = lines.next().ok_or(err())?;
        let reg_b = Regex::new(r"Register B: (\d+)")?
            .captures(l)
            .and_then(|m| Some(m.get(1)?.as_str()))
            .ok_or(err2(l))?
            .parse()?;
        let l = lines.next().ok_or(err())?;
        let reg_c = Regex::new(r"Register C: (\d+)")?
            .captures(l)
            .and_then(|m| Some(m.get(1)?.as_str()))
            .ok_or(err2(l))?
            .parse()?;
        Ok(Self {
            ip: 0,
            reg_a,
            reg_b,
            reg_c,
        })
    }

    fn run(&mut self, program: &Program) -> Result<String> {
        let mut res = vec![];
        while let Some(instr) = program.instrs.get(self.ip) {
            let mut new_ip = self.ip + 2;
            match instr {
                Opcode::Adv(operand) => self.reg_a >>= operand.get_value(&self),
                Opcode::Bxl(LiteralOperand { val }) => self.reg_b ^= val,
                Opcode::Bst(operand) => self.reg_b = operand.get_value(&self) & 7,
                Opcode::Jnz(LiteralOperand { val }) => {
                    if self.reg_a != 0 {
                        let val = *val as usize;
                        new_ip = val; // Should not be that : could land on operand that becomes opcode !
                    }
                }
                Opcode::Bxc(_operand) => self.reg_b ^= self.reg_c,
                Opcode::Out(operand) => res.push(operand.get_value(&self) & 7),
                Opcode::Bdv(operand) => self.reg_b = self.reg_a >> operand.get_value(&self),
                Opcode::Cdv(operand) => self.reg_c = self.reg_a >> operand.get_value(&self),
            }
            self.ip = new_ip;
        }
        Ok(res
            .into_iter()
            .map(|n| n.to_string())
            .fold(String::new(), |mut acc, elem| {
                acc.push_str(&elem);
                acc.push(',');
                acc
            })
            .trim_end_matches(|x| x == ',')
            .to_owned())
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let mut machine = Machine::parse(&mut lines).unwrap();
    println!("{:?}", machine);
    lines.next();
    let program = Program::parse(&mut lines).unwrap();
    println!("{}", program);

    let res = machine.run(&program).unwrap();
    println!("{}", &res);

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let machine = Machine::parse(&mut lines).unwrap();
    println!("{:?}", machine);
    lines.next();
    let (program_string, program) = Program::parse2(&mut lines).unwrap();
    println!("{}", program);

    // let mut start_reg_a = 64_119_171_111_111;
    let mut start_reg_a = 216_169_171_111_111;
    for i in 0..1000 {
        let mut machine_clone = machine.clone();
        machine_clone.reg_a = start_reg_a;
        let res = machine_clone.run(&program).unwrap();
        if res == program_string {
            break;
        }
        println!("Keeping on : A = {}, res = {}", start_reg_a, res);
        // start_reg_a += 1;
        start_reg_a -= 100_000_000;
    }

    Some(start_reg_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
