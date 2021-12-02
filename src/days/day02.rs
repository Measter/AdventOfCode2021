use aoc_lib::{day, Bench, BenchResult, NoError, UserError};
use color_eyre::eyre::{eyre, Result};

day! {
   day 2: "Dive!"
   1: run_part1
   2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instrs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part1(&instrs)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instrs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part2(&instrs)))
}

#[derive(Clone, Copy)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Instruction {
    fn parse(line: &str) -> Result<Self> {
        match line.trim().split_once(' ').map(|(ins, v)| (ins, v.parse())) {
            Some(("forward", Ok(v))) => Ok(Self::Forward(v)),
            Some(("down", Ok(v))) => Ok(Self::Down(v)),
            Some(("up", Ok(v))) => Ok(Self::Up(v)),
            _ => Err(eyre!("invalid instruction: {}", line)),
        }
    }
}

fn part1(instrs: &[Instruction]) -> u32 {
    let (pos, depth) = instrs
        .iter()
        .fold((0, 0), |(pos, depth), &instr| match instr {
            Instruction::Forward(v) => (pos + v, depth),
            Instruction::Down(v) => (pos, depth + v),
            Instruction::Up(v) => (pos, depth.saturating_sub(v)),
        });

    pos * depth
}

fn part2(instrs: &[Instruction]) -> u32 {
    let (_, pos, depth) = instrs
        .iter()
        .fold((0, 0, 0), |(aim, pos, depth), &instr| match instr {
            Instruction::Forward(v) => (aim, pos + v, depth + v * aim),
            Instruction::Down(v) => (aim + v, pos, depth),
            Instruction::Up(v) => (aim - v, pos, depth),
        });

    pos * depth
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(2).example(Example::Part1, 1).open().unwrap();

        let instrs: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(150, part1(&instrs));
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(2).example(Example::Part1, 1).open().unwrap();

        let instrs: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(900, part2(&instrs));
    }
}
