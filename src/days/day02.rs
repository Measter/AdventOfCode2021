use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 2,
    name: "Dive!",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instrs = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&instrs)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instrs = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&instrs)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let instrs = parse(input)?;
        Ok::<_, Report>(ParseResult(instrs))
    })
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

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let instrs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()?;

    Ok(instrs)
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
