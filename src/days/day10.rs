use aoc_lib::{day, Bench, BenchResult, NoError};

// 7:36
// 8:13
// 8:32

day! {
   day 10: "Syntax Scoring"
   1: run_part1
   2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let lines: Vec<_> = input.lines().collect();
    b.bench(|| Ok::<_, NoError>(part1(&lines)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let lines: Vec<_> = input.lines().collect();
    b.bench(|| Ok::<_, NoError>(part2(&lines)))
}

fn bracket_score(byte: u8) -> u64 {
    match byte {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,

        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => 0,
    }
}

fn get_open(byte: u8) -> u8 {
    match byte {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => unreachable!(),
    }
}

fn part1(input: &[&str]) -> u64 {
    let mut stack = Vec::new();
    let mut score = 0;

    for line in input.iter() {
        stack.clear();

        for byte in line.bytes() {
            match byte {
                b'(' | b'[' | b'{' | b'<' => stack.push(byte),
                b')' | b']' | b'}' | b'>' => {
                    if stack.is_empty() {
                        // incomplete?
                        break;
                    }
                    let prev = stack.pop().unwrap();
                    if prev != get_open(byte) {
                        // Corrupted line.
                        score += bracket_score(byte);
                        break;
                    }
                }
                _ => panic!("bad input"),
            }
        }
    }

    score
}

fn part2(input: &[&str]) -> u64 {
    let mut stack = Vec::new();
    let mut scores = Vec::new();

    'outer: for line in input.iter() {
        stack.clear();
        for byte in line.bytes() {
            match byte {
                b'(' | b'[' | b'{' | b'<' => stack.push(byte),
                b')' | b']' | b'}' | b'>' => {
                    let prev = stack.pop().expect("empty stack");
                    if prev != get_open(byte) {
                        // Corrupted line.
                        continue 'outer;
                    }
                }
                _ => panic!("bad input"),
            }
        }

        let mut score = 0;
        while let Some(open) = stack.pop() {
            score *= 5;
            score += bracket_score(open);
        }
        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(10)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let lines: Vec<_> = input.lines().collect();

        assert_eq!(26397, part1(&lines));
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(10)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let lines: Vec<_> = input.lines().collect();

        assert_eq!(288957, part2(&lines));
    }
}
