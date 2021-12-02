use aoc_lib::{day, misc::ArrWindows, Bench, BenchResult, NoError, UserError};

day! {
   day 1: "Sonar Sweep"
   1: run_part1
   2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let depths: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part1(&depths)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let depths: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part2(&depths)))
}

fn part1(depths: &[u32]) -> usize {
    ArrWindows::new(depths).filter(|[a, b]| b > a).count()
}

fn part2(depths: &[u32]) -> usize {
    let windows: Vec<_> = ArrWindows::new(depths).map(|[a, b, c]| a + b + c).collect();

    ArrWindows::new(&windows).filter(|[a, b]| b > a).count()
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
