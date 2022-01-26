use std::collections::HashSet;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult};

//11:32
//12:50 (Part 2)

pub const DAY: Day = Day {
    day: 13,
    name: "Transparent Origami",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (paper, instrs) = PaperInstructionsPart1::parse(input);
    b.bench(|| {
        Ok::<_, NoError>({
            let mut paper = paper.clone();
            paper.make_folds(&instrs[..1]);

            paper.count_dots()
        })
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (paper, instrs) = PaperInstructionsPart2::parse(input);
    b.bench_alt(|| {
        Ok::<_, NoError>({
            let mut paper = paper.clone();

            paper.make_folds(&instrs);

            paper.print_paper()
        })
    })
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(ParseResult(PaperInstructionsPart1::parse(input))))
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(line: &str) -> Point {
        let (x, y) = line.split_once(',').expect("coordinate seperater no found");

        Point {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
        }
    }

    fn reflect(self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Y(y) if self.y > y => {
                let y = 2 * y - self.y;
                Self { y, ..self }
            }
            Instruction::X(x) if self.x > x => {
                let x = 2 * x - self.x;
                Self { x, ..self }
            }
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Y(usize),
    X(usize),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let (axis, val) = line
            .trim()
            .trim_start_matches("fold along ")
            .split_once('=')
            .expect("invalid instruction");

        match axis {
            "y" => Self::Y(val.parse().expect("invalid instruction")),
            "x" => Self::X(val.parse().expect("invalid instruction")),
            _ => panic!("invalid instruction"),
        }
    }
}

#[derive(Debug, Clone)]
struct PaperInstructionsPart1 {
    points: Vec<Point>,
    folded: HashSet<Point>,
    folded_width: usize,
    folded_height: usize,
}

impl PaperInstructionsPart1 {
    fn parse(input: &str) -> (Self, Vec<Instruction>) {
        let (point_str, instr_str) = input.split_once("\n\n").expect("Segment break not found");

        let points: Vec<_> = point_str.trim().lines().map(Point::parse).collect();
        let (width, height) = points
            .iter()
            .fold((0, 0), |(x, y), p| (x.max(p.x + 1), y.max(p.y + 1)));

        let instrs = instr_str.trim().lines().map(Instruction::parse).collect();

        (
            Self {
                points,
                folded: HashSet::new(),
                folded_height: height,
                folded_width: width,
            },
            instrs,
        )
    }

    #[allow(unused)]
    fn print_paper(&self) -> String {
        let mut output = String::new();

        for y in 0..self.folded_height {
            for x in 0..self.folded_width {
                if self.folded.contains(&Point { x, y }) {
                    output.push('#');
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }

        output
    }

    fn make_folds(&mut self, instructions: &[Instruction]) {
        for point in &self.points {
            let new_point = instructions
                .iter()
                .fold(*point, |p, instr| p.reflect(*instr));
            self.folded.insert(new_point);
        }

        let (folded_width, folded_height) = instructions.iter().fold(
            (self.folded_width, self.folded_height),
            |(width, height), instr| match *instr {
                Instruction::Y(y) => (width, y),
                Instruction::X(x) => (x, height),
            },
        );

        self.folded_width = folded_width;
        self.folded_height = folded_height;
    }

    fn count_dots(&self) -> usize {
        self.folded.len()
    }
}

#[derive(Debug, Clone)]
struct PaperInstructionsPart2 {
    points: Vec<Point>,
    folded: Vec<bool>,
    width: usize,
    height: usize,
}

impl PaperInstructionsPart2 {
    fn parse(input: &str) -> (Self, Vec<Instruction>) {
        let (point_str, instr_str) = input.split_once("\n\n").expect("Segment break not found");
        let points: Vec<_> = point_str.trim().lines().map(Point::parse).collect();
        let instrs = instr_str.trim().lines().map(Instruction::parse).collect();

        let (width, height) = points
            .iter()
            .fold((0, 0), |(x, y), p| (x.max(p.x + 1), y.max(p.y + 1)));

        (
            Self {
                points,
                folded: Vec::new(),
                width,
                height,
            },
            instrs,
        )
    }

    fn print_paper(&self) -> String {
        let mut output = String::new();

        for row in self.folded.chunks_exact(self.width) {
            for &p in row {
                output.push(if p { '#' } else { ' ' });
            }
            output.push('\n');
        }

        output
    }

    fn make_folds(&mut self, instructions: &[Instruction]) {
        let (folded_width, folded_height) =
            instructions
                .iter()
                .fold(
                    (self.width, self.height),
                    |(width, height), instr| match *instr {
                        Instruction::Y(y) => (width, y),
                        Instruction::X(x) => (x, height),
                    },
                );

        self.folded.resize(folded_width * folded_height, false);
        self.width = folded_width;
        self.height = folded_height;

        for &point in &self.points {
            let new_point = instructions
                .iter()
                .fold(point, |p, instr| p.reflect(*instr));
            self.folded[new_point.y * folded_width + new_point.x] = true;
        }
    }

    #[allow(unused)]
    fn count_dots(&self) -> usize {
        self.folded.iter().filter(|p| **p).count()
    }
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(13)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let (mut paper, instrs) = PaperInstructionsPart1::parse(&input);
        let mut p1_paper = paper.clone();

        p1_paper.make_folds(&instrs[..1]);
        println!("{}", p1_paper.print_paper());
        assert_eq!(17, p1_paper.count_dots());

        paper.make_folds(&instrs);
        println!("{}", paper.print_paper());
        assert_eq!(16, paper.count_dots());
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(13)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let (mut paper, instrs) = PaperInstructionsPart2::parse(&input);

        let mut p1_paper = paper.clone();

        p1_paper.make_folds(&instrs[..1]);
        println!("{}", p1_paper.print_paper());
        assert_eq!(17, p1_paper.count_dots());

        paper.make_folds(&instrs);
        println!("{}", paper.print_paper());
        assert_eq!(16, paper.count_dots());
    }
}
