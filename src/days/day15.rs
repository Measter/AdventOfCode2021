use std::collections::BinaryHeap;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult};

// 12:03
// 12:53
// 13:08

pub const DAY: Day = Day {
    day: 15,
    name: "Chiton",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse_part1),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let map = Map::parse(input);
    b.bench(|| Ok::<_, NoError>(path_search::<false>(&map)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let map = Map::parse(input);
    b.bench(|| Ok::<_, NoError>(path_search::<true>(&map)))
}

fn run_parse_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(ParseResult(Map::parse(input))))
}

#[derive(Debug)]
struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
    p2_width: usize,
    p2_height: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in input.trim().lines().map(str::trim) {
            width = line.len();
            height += 1;
            for tile in line.chars() {
                tiles.push(tile as u8 - b'0');
            }
        }

        Self {
            tiles,
            width,
            height,
            p2_height: height * 5,
            p2_width: width * 5,
        }
    }

    fn contains<const ISP2: bool>(&self, point: Point) -> bool {
        let (x_range, y_range) = if ISP2 {
            (0..self.p2_width as isize, 0..self.p2_height as isize)
        } else {
            (0..self.width as isize, 0..self.height as isize)
        };

        x_range.contains(&point.x) && y_range.contains(&point.y)
    }

    fn get_cost<const ISP2: bool>(&self, point: Point) -> u64 {
        if !ISP2 {
            let idx = point.y as usize * self.width + point.x as usize;
            self.tiles[idx] as u64
        } else {
            let py = point.y as usize;
            let px = point.x as usize;
            let (real_y, y_tile) = (py % self.height, py / self.height);
            let (real_x, x_tile) = (px % self.height, px / self.height);

            let hazard = self.tiles[real_y * self.width + real_x] as u64;

            ((hazard - 1) + y_tile as u64 + x_tile as u64) % 9 + 1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const INVALID: Point = Point { x: -1, y: -1 };

    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    fn neighbours(self) -> [Point; 4] {
        let Point { x, y } = self;
        [
            Point { x, y: y - 1 },
            Point { x: x - 1, y },
            Point { x: x + 1, y },
            Point { x, y: y + 1 },
        ]
    }

    fn to_idx(self, width: usize) -> usize {
        self.y as usize * width + self.x as usize
    }

    fn estimate_cost(self, target: Self) -> u64 {
        ((target.x - self.x).abs() + (target.y - self.y).abs()) as u64
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct State {
    heuristic_cost: u64,
    cost: u64,
    pos: Point,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic_cost == other.heuristic_cost
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heuristic_cost.cmp(&self.heuristic_cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn path_search<const ISP2: bool>(map: &Map) -> u64 {
    let origin = Point::new(0, 0);
    let mut queue = BinaryHeap::new();

    let (width, height) = if ISP2 {
        (map.p2_width, map.p2_height)
    } else {
        (map.width, map.height)
    };

    let target = Point::new(width - 1, height - 1);
    let mut dist = vec![u64::MAX; width * height];
    let mut prev = vec![Point::INVALID; width * height];

    dist[origin.to_idx(width)] = 0;
    queue.push(State {
        cost: 0,
        heuristic_cost: 0,
        pos: origin,
    });

    while let Some(next) = queue.pop() {
        for neighbour in next.pos.neighbours() {
            if !map.contains::<ISP2>(neighbour) {
                continue;
            }
            let total_cost = next.cost + map.get_cost::<ISP2>(neighbour);
            if neighbour == target {
                return total_cost;
            }
            if total_cost < dist[neighbour.to_idx(width)] {
                dist[neighbour.to_idx(width)] = total_cost;
                prev[neighbour.to_idx(width)] = next.pos;
                queue.push(State {
                    heuristic_cost: total_cost + neighbour.estimate_cost(target),
                    cost: total_cost,
                    pos: neighbour,
                });
            }
        }
    }

    panic!("path not found");
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(15)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let map = Map::parse(&input);
        assert_eq!(40, path_search::<false>(&map));
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(15)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let map = Map::parse(&input);
        assert_eq!(315, path_search::<true>(&map));
    }
}
