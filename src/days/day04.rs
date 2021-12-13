use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Context, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 4,
    name: "Giant Squid",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (numbers, boards) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&numbers, boards.clone())))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (numbers, boards) = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&numbers, boards.clone())))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let game_data = parse(input)?;
        Ok::<_, Report>(ParseResult(game_data))
    })
}

#[derive(Clone)]
struct Board {
    board: [u8; 5 * 5],
    marked: [bool; 5 * 5],
}

impl Board {
    fn mark(&mut self, number: u8) {
        for (n, mark) in self.board.iter().zip(&mut self.marked) {
            if *n == number {
                *mark = true;
            }
        }
    }

    fn check(&self) -> bool {
        let any_row = self.marked.chunks_exact(5).any(|r| r.iter().all(|m| *m));
        if any_row {
            return true;
        }

        for col in 0..5 {
            let all_coll = (0..5)
                .map(|i| i * 5 + col)
                .map(|idx| self.marked[idx])
                .all(|m| m);

            if all_coll {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.board
            .iter()
            .zip(self.marked)
            .filter(|&(_, marked)| !marked)
            .map(|(val, _)| *val as u32)
            .sum()
    }
}

fn parse(input: &str) -> Result<(Vec<u8>, Vec<Board>)> {
    let mut sections = input.split("\r\n\r\n");

    let numbers: Vec<_> = sections
        .next()
        .ok_or_else(|| eyre!("parse error"))?
        .split(',')
        .map(str::trim)
        .map(str::parse::<u8>)
        .collect::<Result<_, _>>()
        .with_context(|| eyre!("invalid number list"))?;

    let mut boards = Vec::new();
    for section in sections {
        let mut board_nums = [0; 5 * 5];
        let section_nums = section.split_whitespace().map(str::parse);
        for (num, parsed) in board_nums.iter_mut().zip(section_nums) {
            *num = parsed.with_context(|| eyre!("invalid board"))?;
        }

        boards.push(Board {
            board: board_nums,
            marked: Default::default(),
        });
    }

    Ok((numbers, boards))
}

fn part1(numbers: &[u8], mut boards: Vec<Board>) -> u32 {
    for &number in numbers {
        boards.iter_mut().for_each(|b| b.mark(number));

        if let Some(winner) = boards.iter().find(|board| board.check()) {
            return winner.sum_unmarked() * number as u32;
        }
    }

    panic!("No winners");
}

fn part2(numbers: &[u8], mut boards: Vec<Board>) -> u32 {
    let mut numbers_iter = numbers.iter();
    while let Some(&number) = numbers_iter.next() {
        boards.iter_mut().for_each(|b| b.mark(number));
        boards.retain(|board| !board.check());

        if let [winner] = boards.as_mut_slice() {
            for &number in numbers_iter.by_ref() {
                winner.mark(number);
                if winner.check() {
                    return winner.sum_unmarked() * number as u32;
                }
            }
        }
    }

    panic!("No winners");
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(4).example(Example::Part1, 1).open().unwrap();
        let (numbers, boards) = parse(&input).unwrap();

        assert_eq!(4512, part1(&numbers, boards));
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(4).example(Example::Part1, 1).open().unwrap();
        let (numbers, boards) = parse(&input).unwrap();

        assert_eq!(1924, part2(&numbers, boards));
    }
}
