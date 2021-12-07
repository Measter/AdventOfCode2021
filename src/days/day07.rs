use aoc_lib::{day, Bench, BenchResult, NoError, UserError};

day! {
   day 7: "The Treachery of Whales"
   1: run_part1
   2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let crabs: Vec<_> = input
        .split_terminator(',')
        .map(str::trim)
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(find_fuel(&crabs, part1_fuel)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let crabs: Vec<_> = input
        .split_terminator(',')
        .map(str::trim)
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(find_fuel(&crabs, part2_fuel)))
}

fn part1_fuel(n: u32) -> u32 {
    n
}

fn part2_fuel(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

fn find_fuel(crabs: &[u32], fuel_cost: impl Fn(u32) -> u32) -> u32 {
    let max_crab = crabs.iter().max().copied().unwrap() as usize + 1;
    let mut crab_by_pos = vec![0; max_crab];
    for &crab in crabs {
        crab_by_pos[crab as usize] += 1;
    }

    (0..max_crab)
        .map(|i| {
            let left_fuel: u32 = crab_by_pos[..i]
                .iter()
                .enumerate()
                .map(|(pos, &c)| {
                    let n = i - pos;
                    let fuel = fuel_cost(n as u32);
                    fuel as u32 * c
                })
                .sum();

            let right_fuel = crab_by_pos
                .get((i + 1)..)
                .map(|sbs| {
                    sbs.iter()
                        .zip((i + 1)..)
                        .map(|(&c, pos)| {
                            let n = pos - i;
                            let fuel = fuel_cost(n as u32);
                            fuel as u32 * c
                        })
                        .sum()
                })
                .unwrap_or(u32::MAX);

            left_fuel.saturating_add(right_fuel)
        })
        .min()
        .unwrap()
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

        assert_eq!(37, find_fuel(&crabs, part1_fuel));
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

        assert_eq!(168, find_fuel(&crabs, part2_fuel));
    }
}
