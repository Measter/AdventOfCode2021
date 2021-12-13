use std::collections::HashSet;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult};

//11:32
//12:50 (Part 2)

pub const DAY: Day = Day {
    day: 13,
    name: "Transparent Origami",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: Vec::new(),
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (paper, instrs) = PaperInstructions::parse(input);
    b.bench(|| {
        Ok::<_, NoError>({
            let mut paper = paper.clone();
            paper.make_fold(instrs[0]);

            paper.count_dots()
        })
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (paper, instrs) = PaperInstructions::parse(input);
    b.bench_alt(|| {
        Ok::<_, NoError>({
            let mut paper = paper.clone();

            for &instr in &instrs {
                paper.make_fold(instr);
            }

            paper.print_paper()
        })
    })
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(ParseResult(PaperInstructions::parse(input))))
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
struct PaperInstructions {
    dot_field: HashSet<Point>,
    buffer: HashSet<Point>,
    folded_width: usize,
    folded_height: usize,
}

impl PaperInstructions {
    fn parse(input: &str) -> (Self, Vec<Instruction>) {
        let (point_str, instr_str) = input.split_once("\n\n").expect("Segment break not found");

        let points: HashSet<_> = point_str.trim().lines().map(Point::parse).collect();
        let (width, height) = points
            .iter()
            .fold((0, 0), |(x, y), p| (x.max(p.x + 1), y.max(p.y + 1)));

        let instrs = instr_str.trim().lines().map(Instruction::parse).collect();

        (
            Self {
                dot_field: points,
                buffer: HashSet::new(),
                folded_height: height,
                folded_width: width,
            },
            instrs,
        )
    }

    fn print_paper(&self) -> String {
        let mut output = String::new();

        for y in 0..self.folded_height {
            for x in 0..self.folded_width {
                if self.dot_field.contains(&Point { x, y }) {
                    output.push('#');
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }

        output
    }

    fn make_fold(&mut self, instruction: Instruction) {
        self.buffer.clear();
        for point in self.dot_field.drain() {
            let mirrored = point.reflect(instruction);
            self.buffer.insert(mirrored);
        }

        match instruction {
            Instruction::Y(y) => self.folded_height = y,
            Instruction::X(x) => self.folded_width = x,
        }

        std::mem::swap(&mut self.buffer, &mut self.dot_field);
    }

    fn count_dots(&self) -> usize {
        self.dot_field.len()
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

        let (mut paper, instrs) = PaperInstructions::parse(&input);

        paper.make_fold(instrs[0]);
        println!("{}", paper.print_paper());
        assert_eq!(17, paper.count_dots());

        paper.make_fold(instrs[1]);
        println!("{}", paper.print_paper());
        assert_eq!(16, paper.count_dots());
    }
}
