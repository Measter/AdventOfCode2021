use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Context, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 6,
    name: "Lanternfish",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let shoal = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(shoal, 80)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let shoal = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(shoal, 256)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let shoal = parse(input)?;
        Ok::<_, Report>(ParseResult(shoal))
    })
}

fn parse(input: &str) -> Result<[u64; 9]> {
    let mut shoal = [0; 9];

    for fish in input.split_terminator(',') {
        let fish: usize = fish
            .trim()
            .parse()
            .with_context(|| eyre!("invalid fish: {}", fish))?;

        *shoal
            .get_mut(fish)
            .ok_or_else(|| eyre!("invalid fish: {}", fish))? += 1;
    }

    Ok(shoal)
}

fn part1(mut shoal: [u64; 9], ticks: u16) -> u64 {
    for _ in 0..ticks {
        let start = shoal[0];
        shoal[0] = shoal[1];
        shoal[1] = shoal[2];
        shoal[2] = shoal[3];
        shoal[3] = shoal[4];
        shoal[4] = shoal[5];
        shoal[5] = shoal[6];
        shoal[6] = shoal[7] + start;
        shoal[7] = shoal[8];
        shoal[8] = start;
    }

    shoal.iter().sum()
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::{input, Example};

    #[test]
    fn part1_example1() {
        let mut shoal = [0; 9];
        shoal[3] = 1;
        assert_eq!(2, part1(shoal, 5));
    }

    #[test]
    fn part1_example2() {
        let input = input(6).example(Example::Part1, 1).open().unwrap();
        let shoal = parse(&input).unwrap();

        assert_eq!(5934, part1(shoal, 80));
    }
}
