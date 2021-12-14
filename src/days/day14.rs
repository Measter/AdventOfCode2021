use std::collections::HashMap;

use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day, NoError, ParseResult};

// 13:15
// 14:08 - Part 2

pub const DAY: Day = Day {
    day: 14,
    name: "Extended Polymerization",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (template, rules) = parse(input);
    b.bench(|| Ok::<_, NoError>(run_replace::<10>(&template, &rules)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (template, rules) = parse(input);
    b.bench(|| Ok::<_, NoError>(run_replace::<40>(&template, &rules)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(ParseResult(parse(input))))
}

fn parse(input: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let (template, rest) = input.trim().split_once('\n').expect("bad input");

    let mut rules = HashMap::new();
    for line in rest.trim().lines() {
        let (pattern, insertion) = line.trim().split_once(" -> ").expect("bad input");
        assert!(insertion.len() == 1);
        let pattern: [u8; 2] = pattern.as_bytes().try_into().unwrap();
        if rules
            .insert(pattern.map(|b| b as char), insertion.as_bytes()[0] as char)
            .is_some()
        {
            panic!("duplicate rule");
        }
    }

    (template.chars().collect(), rules)
}

fn run_replace<const N: usize>(template: &[char], rules: &HashMap<[char; 2], char>) -> usize {
    let mut pairs = HashMap::<[char; 2], usize>::new();

    for &pair in ArrWindows::new(template) {
        *pairs.entry(pair).or_default() += 1;
    }

    *pairs
        .entry([template[template.len() - 1], '\0'])
        .or_default() += 1;

    let mut dst = HashMap::<[char; 2], usize>::new();

    for _ in 0..N {
        dst.clear();
        for (key @ [a, b], count) in pairs.drain() {
            if let Some(&replace) = rules.get(&key) {
                *dst.entry([a, replace]).or_default() += count;
                *dst.entry([replace, b]).or_default() += count;
            } else {
                dst.insert(key, count);
            }
        }

        std::mem::swap(&mut pairs, &mut dst);
    }

    let mut counts = [0_usize; 26];
    for ([b, _], count) in pairs {
        counts[(b as u8 - b'A') as usize] += count;
    }

    let (min, max) = counts.into_iter().fold((usize::MAX, 0), |(min, max), b| {
        (if b > 0 { min.min(b) } else { min }, max.max(b))
    });

    max - min
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(14)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let (template, rules) = parse(&input);
        assert_eq!(1588, run_replace::<10>(&template, &rules));
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(14)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let (template, rules) = parse(&input);
        assert_eq!(2188189693529, run_replace::<40>(&template, &rules));
    }
}
