use std::cmp::Ordering;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 5,
    name: "Hydrothermal Venture",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: Vec::new(),
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let lines = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&lines)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let lines = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&lines)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let lines = parse(input)?;
        Ok::<_, Report>(ParseResult(lines))
    })
}

fn parse(input: &str) -> Result<Vec<Line>> {
    let lines: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Line::parse)
        .collect::<Result<_, _>>()?;

    Ok(lines)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn parse(val: &str) -> Result<Self> {
        let (x, y) = val
            .split_once(',')
            .ok_or_else(|| eyre!("Invalid coordinate pair: {}", val))?;

        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse(line: &str) -> Result<Self> {
        let (left, right) = line
            .split_once(" -> ")
            .ok_or_else(|| eyre!("Invalid line: {}", line))?;

        Ok(Self {
            start: Point::parse(left)?,
            end: Point::parse(right)?,
        })
    }
}
fn part1(lines: &[Line]) -> usize {
    let mut point_map = vec![0u8; 1000 * 1000];
    let mut high_points = 0;

    for line in lines
        .iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
    {
        if line.start.x == line.end.x {
            // Vertical line!
            let lower = line.start.y.min(line.end.y);
            let upper = line.start.y.max(line.end.y);
            (lower..=upper).for_each(|y| {
                let idx = y as usize * 1000 + line.start.x as usize;
                let new_val = point_map[idx].saturating_add(1);
                point_map[idx] = new_val;
                high_points += (new_val == 2) as usize;
            });
        } else {
            // Horizontal line!
            let lower = line.start.x.min(line.end.x);
            let upper = line.start.x.max(line.end.x);
            (lower..=upper).for_each(|x| {
                let idx = line.start.y as usize * 1000 + x as usize;
                let new_val = point_map[idx].saturating_add(1);
                point_map[idx] = new_val;
                high_points += (new_val == 2) as usize;
            });
        }
    }

    high_points
}

fn part2(lines: &[Line]) -> usize {
    let mut point_map = vec![0u8; 1000 * 1000];
    let mut high_points = 0;

    for line in lines.iter() {
        let dy = match line.start.y.cmp(&line.end.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => u16::MAX,
        };
        let dx = match line.start.x.cmp(&line.end.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => u16::MAX,
        };

        let mut x = line.start.x;
        let mut y = line.start.y;

        loop {
            let idx = y as usize * 1000 + x as usize;
            let new_val = unsafe {
                let v = point_map.get_unchecked_mut(idx);
                *v = v.saturating_add(1);
                *v
            };
            high_points += (new_val == 2) as usize;
            if x == line.end.x && y == line.end.y {
                break;
            }

            x = x.wrapping_add(dx);
            y = y.wrapping_add(dy);
        }
    }

    high_points
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(5).example(Example::Part1, 1).open().unwrap();

        let lines: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Line::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(5, part1(&lines));
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(5).example(Example::Part1, 1).open().unwrap();

        let lines: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Line::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(12, part2(&lines));
    }
}
