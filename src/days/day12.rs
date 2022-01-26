use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult};
use lasso::{Key, Rodeo, Spur};

// 8:42
// 9:30
// 9:48

pub const DAY: Day = Day {
    day: 12,
    name: "Passage Pathing",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
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

#[derive(Default)]
struct Cave {
    is_big: bool,
    leads_to: Vec<Spur>,
}

struct CaveSystem {
    entry: Spur,
    exit: Spur,
    caves: Vec<Cave>,
    interner: Rodeo,
}

impl CaveSystem {
    fn get_cave(caves: &mut Vec<Cave>, id: Spur) -> &mut Cave {
        let idx = id.into_usize();
        if idx >= caves.len() {
            caves.resize_with(idx + 1, Default::default);
        }
        &mut caves[idx]
    }

    fn parse(input: &str) -> CaveSystem {
        let mut entry = None;
        let mut exit = None;
        let mut caves = Vec::new();
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

            let start_cave = CaveSystem::get_cave(&mut caves, start_spur);
            start_cave.is_big = start.chars().all(|c| c.is_uppercase());
            start_cave.leads_to.push(end_spur);

            let end_cave = CaveSystem::get_cave(&mut caves, end_spur);
            end_cave.is_big = end.chars().all(|c| c.is_uppercase());
            end_cave.leads_to.push(start_spur);
        }

        CaveSystem {
            entry: entry.expect("no start found"),
            exit: exit.expect("no end found"),
            caves,
            interner,
        }
    }

    fn traverse_paths_part1(&self, root: Spur, visited: &mut [bool]) -> usize {
        if root == self.exit {
            return 1;
        }
        let cave = &self.caves[root.into_usize()];

        if !cave.is_big {
            if visited[root.into_usize()] {
                return 0;
            }
            visited[root.into_usize()] = true;
        }

        let mut num_paths = 0;
        for &next_cave in &cave.leads_to {
            num_paths += self.traverse_paths_part1(next_cave, visited);
        }

        if !cave.is_big {
            visited[root.into_usize()] = false;
        }

        num_paths
    }

    fn traverse_paths_part2(
        &self,
        root: Spur,
        mut visited_twice: bool,
        visited: &mut [u16],
    ) -> usize {
        if root == self.exit {
            return 1;
        }
        let cave = &self.caves[root.into_usize()];
        let visit_count = visited[root.into_usize()];
        if root == self.entry || (!cave.is_big && visited_twice && visit_count >= 1) {
            return 0;
        }

        if !cave.is_big {
            visited[root.into_usize()] += 1;
            visited_twice |= visited[root.into_usize()] >= 2;
        }
        let mut num_paths = 0;
        for &next_cave in &cave.leads_to {
            num_paths += self.traverse_paths_part2(next_cave, visited_twice, visited);
        }

        if !cave.is_big {
            visited[root.into_usize()] -= 1;
        }

        num_paths
    }
}

fn part1(cave_system: &CaveSystem) -> usize {
    let entry_cave = &cave_system.caves[cave_system.entry.into_usize()];

    let mut visited = vec![false; cave_system.interner.len()];
    visited[cave_system.entry.into_usize()] = true;

    let mut num_paths = 0;
    for &next_cave in &entry_cave.leads_to {
        num_paths += cave_system.traverse_paths_part1(next_cave, &mut visited);
    }

    num_paths
}

fn part2(cave_system: &CaveSystem) -> usize {
    let entry_cave = &cave_system.caves[cave_system.entry.into_usize()];

    let mut visited = vec![0; cave_system.interner.len() + 1];
    visited[cave_system.entry.into_usize()] = 1;

    let mut num_paths = 0;
    for &next_cave in &entry_cave.leads_to {
        num_paths += cave_system.traverse_paths_part2(next_cave, false, &mut visited);
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
