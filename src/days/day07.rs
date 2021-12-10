use aoc_lib::{Bench, BenchResult, Day, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 7,
    name: "The Treachery of Whales",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: Vec::new(),
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let crabs: Vec<_> = parse(input).map_err(UserError)?;
    b.bench(|| find_fuel(&crabs, part1_fuel))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let crabs: Vec<_> = parse(input).map_err(UserError)?;

    b.bench(|| find_fuel(&crabs, part2_fuel))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let crabs = parse(input)?;
        Ok::<_, Report>(ParseResult(crabs))
    })
}

fn parse(input: &str) -> Result<Vec<u32>> {
    let crabs: Vec<_> = input
        .split_terminator(',')
        .map(str::trim)
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()?;

    Ok(crabs)
}

fn part1_fuel(n: u32) -> u32 {
    n
}

fn part2_fuel(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

#[derive(Debug, Clone, Copy)]
struct Position {
    pos: u32,
    num_crabs: u32,
}

fn find_fuel(crabs: &[u32], fuel_cost: impl Fn(u32) -> u32) -> Result<u32, &'static str> {
    let mut crab_by_pos: Vec<Position> = Vec::new();
    for crab in crabs {
        match crab_by_pos.binary_search_by(|cp| cp.pos.cmp(crab)) {
            Ok(idx) => crab_by_pos[idx].num_crabs += 1,
            Err(idx) => crab_by_pos.insert(
                idx,
                Position {
                    pos: *crab,
                    num_crabs: 1,
                },
            ),
        }
    }

    let last_crab = crab_by_pos.last().unwrap().pos;
    let mut cur_idx = 0;
    let mut cur_fuel = u32::MAX;

    for pos in 0..=last_crab {
        if crab_by_pos[cur_idx].pos == pos {
            cur_idx += 1;
        }

        let right_fuel = crab_by_pos[cur_idx..]
            .iter()
            .map(|crab_pos| {
                let n = crab_pos.pos - pos;
                fuel_cost(n) * crab_pos.num_crabs
            })
            .sum();
        let left_fuel: u32 = crab_by_pos[..cur_idx]
            .iter()
            .map(|crab_pos| {
                let n = pos - crab_pos.pos;
                fuel_cost(n) * crab_pos.num_crabs
            })
            .sum();

        let total_fuel = left_fuel.saturating_add(right_fuel);
        if total_fuel > cur_fuel {
            return Ok(cur_fuel);
        }
        cur_fuel = total_fuel;

        if crab_by_pos[cur_idx].pos == pos {}
    }

    Err("No minimum fuel!?")
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(7).example(Example::Part1, 1).open().unwrap();
        let crabs: Vec<_> = input
            .split_terminator(',')
            .map(str::trim)
            .map(str::parse::<u32>)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(37, find_fuel(&crabs, part1_fuel).unwrap());
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(7).example(Example::Part1, 1).open().unwrap();
        let crabs: Vec<_> = input
            .split_terminator(',')
            .map(str::trim)
            .map(str::parse::<u32>)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(168, find_fuel(&crabs, part2_fuel).unwrap());
    }
}
