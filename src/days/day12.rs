use std::collections::{HashMap, HashSet};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult};
use color_eyre::eyre::{eyre, Result};
use lasso::{Interner, Rodeo, Spur};

// 8:42
// 9:30
// 9:48

pub const DAY: Day = Day {
    day: 12,
    name: "Passage Pathing",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: Vec::new(),
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let cave_system = CaveSystem::parse(input);
    b.bench(|| Ok::<_, NoError>(part1(&cave_system)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let cave_system = CaveSystem::parse(input);
    b.bench(|| Ok::<_, NoError>(part2(&cave_system)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(ParseResult(CaveSystem::parse(input))))
}

struct Cave {
    is_big: bool,
    leads_to: Vec<Spur>,
}

struct CaveSystem {
    entry: Spur,
    exit: Spur,
    caves: HashMap<Spur, Cave>,
    interner: Rodeo,
}

impl CaveSystem {
    fn parse(input: &str) -> CaveSystem {
        let mut entry = None;
        let mut exit = None;
        let mut caves = HashMap::new();
        let mut interner = Rodeo::new();

        for line in input.trim().lines() {
            let (start, end) = line.split_once('-').expect("invalid line");
            let start_spur = interner.get_or_intern(start);
            let end_spur = interner.get_or_intern(end);

            if start == "start" {
                entry = Some(start_spur);
            } else if end == "start" {
                entry = Some(end_spur);
            } else if start == "end" {
                exit = Some(start_spur);
            } else if end == "end" {
                exit = Some(end_spur);
            }

            caves
                .entry(start_spur)
                .or_insert_with(|| Cave {
                    is_big: start.chars().all(|c| c.is_uppercase()),
                    leads_to: Vec::new(),
                })
                .leads_to
                .push(end_spur);

            caves
                .entry(end_spur)
                .or_insert_with(|| Cave {
                    is_big: end.chars().all(|c| c.is_uppercase()),
                    leads_to: Vec::new(),
                })
                .leads_to
                .push(start_spur);
        }

        CaveSystem {
            entry: entry.expect("no start found"),
            exit: exit.expect("no end found"),
            caves,
            interner,
        }
    }

    fn print_path(&self, path: Vec<Spur>) {
        match &*path {
            [] => {}
            [start] => {
                println!("{}", self.interner.resolve(start));
            }
            [start, rest @ ..] => {
                print!("{}", self.interner.resolve(start));
                for node in rest {
                    print!(", {}", self.interner.resolve(node));
                }
                println!();
            }
        }
    }

    fn traverse_paths_part1(
        &self,
        root: Spur,
        mut path: Vec<Spur>,
        mut visited: HashSet<Spur>,
    ) -> usize {
        path.push(root);
        if root == self.exit {
            // self.print_path(path);
            return 1;
        }
        let cave = &self.caves[&root];
        if !cave.is_big && !visited.insert(root) {
            return 0;
        }

        let mut num_paths = 0;
        for &next_cave in &cave.leads_to {
            num_paths += self.traverse_paths_part1(next_cave, path.clone(), visited.clone());
        }

        num_paths
    }

    fn traverse_paths_part2(
        &self,
        root: Spur,
        mut path: Vec<Spur>,
        mut visited_twice: bool,
        mut visited: HashMap<Spur, u8>,
    ) -> usize {
        path.push(root);
        if root == self.exit {
            // self.print_path(path);
            return 1;
        }
        let cave = &self.caves[&root];
        let visit_count = *visited.get(&root).unwrap_or(&0);
        if root == self.entry
            || (!cave.is_big && visited_twice && visit_count == 1)
            || (!cave.is_big && visit_count == 2)
        {
            return 0;
        }

        if !cave.is_big {
            let count = visited.entry(root).or_insert(0);
            *count += 1;
            if *count == 2 {
                visited_twice = true;
            }
        }

        let mut num_paths = 0;
        for &next_cave in &cave.leads_to {
            num_paths +=
                self.traverse_paths_part2(next_cave, path.clone(), visited_twice, visited.clone());
        }

        num_paths
    }
}

fn part1(cave_system: &CaveSystem) -> usize {
    let mut visited = HashSet::new();

    let entry_cave = cave_system.caves.get(&cave_system.entry).unwrap();
    visited.insert(cave_system.entry);

    let mut num_paths = 0;
    let mut path = vec![cave_system.entry];
    for &next_cave in &entry_cave.leads_to {
        num_paths += cave_system.traverse_paths_part1(next_cave, path.clone(), visited.clone());
    }

    num_paths
}

fn part2(cave_system: &CaveSystem) -> usize {
    let mut visited = HashMap::new();

    let entry_cave = cave_system.caves.get(&cave_system.entry).unwrap();
    visited.insert(cave_system.entry, 1);

    let mut num_paths = 0;
    let mut path = vec![cave_system.entry];
    for &next_cave in &entry_cave.leads_to {
        num_paths +=
            cave_system.traverse_paths_part2(next_cave, path.clone(), false, visited.clone());
    }

    num_paths
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(12)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let cave_system = CaveSystem::parse(&input);
        let num_paths = part1(&cave_system);
        assert_eq!(10, num_paths);
    }

    #[test]
    fn part1_test2() {
        let input = aoc_lib::input(12)
            .example(Example::Part1, 2)
            .open()
            .unwrap();

        let cave_system = CaveSystem::parse(&input);
        let num_paths = part1(&cave_system);
        assert_eq!(19, num_paths);
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(12)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let cave_system = CaveSystem::parse(&input);
        let num_paths = part2(&cave_system);
        assert_eq!(36, num_paths);
    }
}
