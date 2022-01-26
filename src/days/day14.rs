use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day, NoError, ParseResult};

// 13:15
// 14:08 - Part 2

pub const DAY: Day = Day {
    day: 14,
    name: "Extended Polymerization",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
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

fn get_idx(pair: [u8; 2]) -> usize {
    let pair = pair.map(|b| b as usize);
    pair[0] * 32 + pair[1]
}

const NO_RULE: u8 = 255;

fn parse(input: &str) -> (Vec<u8>, Vec<u8>) {
    let (template, rest) = input.trim().split_once('\n').expect("bad input");

    let mut rules_lookup = vec![NO_RULE; 32 * 32];
    for line in rest.trim().lines() {
        let (pattern, insertion) = line.trim().split_once(" -> ").expect("bad input");
        assert!(insertion.len() == 1);
        let [a, b]: [u8; 2] = pattern.as_bytes().try_into().expect("bad input");
        let pattern_idx = get_idx([a - b'A', b - b'A']);

        rules_lookup[pattern_idx] = insertion.as_bytes()[0] - b'A';
    }

    let template = template.bytes().map(|b| b - b'A').collect();
    (template, rules_lookup)
}

fn run_replace<const N: usize>(template: &[u8], rules: &[u8]) -> usize {
    let mut pairs = vec![0_usize; 32 * 32];

    for &pair in ArrWindows::new(template) {
        let idx = get_idx(pair);
        pairs[idx] += 1;
    }

    pairs[get_idx([template[template.len() - 1], 26])] += 1;

    let mut dst = vec![0_usize; 32 * 32];

    for _ in 0..N {
        dst.fill(0);
        for (pair, &count) in pairs.iter().enumerate() {
            if count == 0 {
                continue;
            }
            let (a, b) = (pair / 32, pair % 32);
            let replace = unsafe { *rules.get_unchecked(pair) };
            if replace != NO_RULE {
                let a_idx = get_idx([a as u8, replace]);
                let b_idx = get_idx([replace, b as u8]);
                unsafe {
                    *dst.get_unchecked_mut(a_idx) += count;
                    *dst.get_unchecked_mut(b_idx) += count;
                }
            } else {
                unsafe { *dst.get_unchecked_mut(pair) = count };
            }
        }

        std::mem::swap(&mut pairs, &mut dst);
    }

    let mut counts = [0_usize; 32];
    for (pair, count) in pairs.into_iter().enumerate() {
        counts[pair / 32] += count;
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
