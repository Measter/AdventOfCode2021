use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

//8:18
//9:23
//9:27

pub const DAY: Day = Day {
    day: 11,
    name: "Dumbo Octopus",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: Vec::new(),
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let field = Field::parse(input.trim()).map_err(UserError)?;

    b.bench(|| {
        let mut field = field.clone();
        let mut check_queue = Vec::new();
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += field.step(&mut check_queue);
        }

        Ok::<_, NoError>(flashes)
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let field = Field::parse(input.trim()).map_err(UserError)?;
    b.bench(|| {
        let mut field = field.clone();
        let mut step_queue = Vec::new();
        let mut flash_step = 0;
        for i in 1..=5000 {
            if field.step(&mut step_queue) == field.tiles.len() as u64 {
                flash_step = i;
                break;
            }
        }
        Ok::<_, NoError>(flash_step)
    })
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let field = Field::parse(input)?;
        Ok::<_, Report>(ParseResult(field))
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

fn get_neighbours(idx: usize, width: usize, height: usize) -> [Option<usize>; 8] {
    let x = (idx % width) as isize;
    let y = (idx / width) as isize;
    let iwidth = width as isize;
    let iheight = height as isize;
    #[rustfmt::skip]
    let rel: [(isize, isize); 8] = [
        (-1, -1), ( 0, -1), ( 1, -1),
        (-1,  0),           ( 1,  0),
        (-1,  1), ( 0,  1), ( 1,  1)
    ];

    rel.map(|(rx, ry)| {
        let new_x = match (x, rx) {
            (0, -1) => return None,
            (_, 1) if x == iwidth - 1 => return None,
            (x, rx) => (x + rx) as usize,
        };

        let new_y = match (y, ry) {
            (0, -1) => return None,
            (_, 1) if y == iheight - 1 => return None,
            (y, ry) => (y + ry) as usize,
        };

        Some(new_y * width + new_x)
    })
}

impl Field {
    fn parse(input: &str) -> Result<Field> {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            let line = line.trim();
            width = line.len();
            height += 1;
            tiles.extend(line.bytes().map(|b| b - b'0'));
        }

        Ok(Field {
            tiles,
            width,
            height,
        })
    }

    fn step(&mut self, check_queue: &mut Vec<usize>) -> u64 {
        self.tiles.iter_mut().for_each(|t| *t += 1);

        check_queue.clear();
        for idx in 0..self.tiles.len() {
            if self.tiles[idx] < 10 || self.tiles[idx] & 0x80 != 0 {
                continue;
            }

            self.tiles[idx] |= 0x80; // Mark it as flashed.

            let neighbours = get_neighbours(idx, self.width, self.height);
            check_queue.extend(neighbours.into_iter().flatten());

            while let Some(nb) = check_queue.pop() {
                let mut new_val = self.tiles[nb].saturating_add(1);

                // If it's hit the trigger, but hasn't flashed.
                if new_val >= 10 && new_val & 0x80 == 0 {
                    let neighbours = get_neighbours(nb, self.width, self.height);
                    check_queue.extend(neighbours.into_iter().flatten());
                    // Mark it as flashed;
                    new_val |= 0x80;
                }

                self.tiles[nb] = new_val;
            }
        }

        let mut flashes = 0;
        for tile in &mut self.tiles {
            if *tile >= 10 {
                flashes += 1;
                *tile = 0;
            }
        }

        flashes
    }

    fn print_table(&self) {
        for row in self.tiles.chunks_exact(self.width) {
            for tile in row {
                print!("{}", tile);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test1() {
        let input = aoc_lib::input(11)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let mut field = Field::parse(&input).unwrap();
        let mut check_queue = Vec::new();

        let mut flashes = 0;
        for _ in 0..100 {
            flashes += field.step(&mut check_queue);
        }

        assert_eq!(1656, flashes);
    }

    #[test]
    fn part1_test2() {
        let input = aoc_lib::input(11)
            .example(Example::Part1, 2)
            .open()
            .unwrap();

        let (start, end) = input.split_once("-----").unwrap();
        let mut start_field = Field::parse(start.trim()).unwrap();
        let end_field = Field::parse(end.trim()).unwrap();
        let mut check_queue = Vec::new();

        let mut flashes = 0;
        for _ in 0..2 {
            flashes += start_field.step(&mut check_queue);
        }

        assert_eq!(9, flashes);
        assert_eq!(start_field, end_field);
    }

    #[test]
    fn part2_test1() {
        let input = aoc_lib::input(11)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let mut field = Field::parse(&input).unwrap();
        let mut check_queue = Vec::new();

        let mut flash_step = 0;
        for i in 1..=200 {
            if field.step(&mut check_queue) == field.tiles.len() as u64 {
                flash_step = i;
                break;
            }
        }

        assert_eq!(195, flash_step);
    }
}
