use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::Result, Report};

pub const DAY: Day = Day {
    day: 1,
    name: "Sonar Sweep",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: Vec::new(),
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let depths = parse(input).map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part1(&depths)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let depths = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&depths)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let depths = parse(input)?;
        Ok::<_, Report>(ParseResult(depths))
    })
}

fn parse(input: &str) -> Result<Vec<u32>> {
    let depths: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    Ok(depths)
}

fn part1(depths: &[u32]) -> usize {
    ArrWindows::new(depths).filter(|[a, b]| b > a).count()
}

fn part2(depths: &[u32]) -> usize {
    ArrWindows::new(depths)
        .filter(|[a, b, c, d]| b + c + d > a + b + c)
        .count()
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(1).example(Example::Part1, 1).open().unwrap();

        let data: Vec<_> = data
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let result = part1(&data);

        assert_eq!(7, result);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(1).example(Example::Part1, 1).open().unwrap();

        let data: Vec<_> = data
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let result = part2(&data);

        assert_eq!(5, result);
    }
}
