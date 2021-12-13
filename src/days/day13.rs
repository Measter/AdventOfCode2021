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

#[derive(Debug, Clone, Copy)]
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
    dot_field: Vec<bool>,
    width: usize,
    folded_width: usize,
    folded_height: usize,
}

impl PaperInstructions {
    fn parse(input: &str) -> (PaperInstructions, Vec<Instruction>) {
        let (point_str, instr_str) = input.split_once("\n\n").expect("Segment break not found");

        let points: Vec<_> = point_str.trim().lines().map(Point::parse).collect();
        let (width, height) = points
            .iter()
            .fold((0, 0), |(x, y), p| (x.max(p.x + 1), y.max(p.y + 1)));

        let mut dot_field = vec![false; width * height];
        points
            .into_iter()
            .for_each(|p| dot_field[p.y * width + p.x] = true);

        let instrs = instr_str.trim().lines().map(Instruction::parse).collect();

        (
            Self {
                dot_field,
                width,
                folded_height: height,
                folded_width: width,
            },
            instrs,
        )
    }

    fn print_paper(&self) -> String {
        let mut output = String::new();
        for row in self
            .dot_field
            .chunks_exact(self.width)
            .take(self.folded_height)
        {
            for tile in &row[..self.folded_width] {
                if *tile {
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
        match instruction {
            Instruction::Y(y) => {
                let row_start = y * self.width;
                let dot_field = &mut self.dot_field[..self.folded_height * self.width];
                let (top, bottom) = dot_field.split_at_mut(row_start);

                top.chunks_exact_mut(self.width)
                    .rev()
                    .zip(bottom[self.width..].chunks_exact(self.width))
                    .for_each(|(top, bottom)| {
                        top[..self.folded_width]
                            .iter_mut()
                            .zip(bottom)
                            .for_each(|(t, &b)| *t |= b);
                    });

                self.folded_height = y;
            }
            Instruction::X(x) => {
                for row in
                    self.dot_field[..self.folded_height * self.width].chunks_exact_mut(self.width)
                {
                    let (left, right) = row.split_at_mut(x);

                    left.iter_mut()
                        .rev()
                        .zip(&right[1..])
                        .for_each(|(l, &r)| *l |= r);
                }

                self.folded_width = x;
            }
        }
    }

    fn count_dots(&self) -> usize {
        self.dot_field[..self.folded_height * self.width]
            .chunks_exact(self.width)
            .map(|row| row[..self.folded_width].iter().filter(|p| **p).count())
            .sum()
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
