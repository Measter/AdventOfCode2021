use std::collections::BTreeSet;

use aoc_lib::{day, Bench, BenchResult, NoError, UserError};
use color_eyre::eyre::{eyre, Result};

day! {
   day 8: "Seven Segment Search"
   1: run_part1
   2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Data::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Data::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

#[derive(Debug, Clone, Copy)]
struct Data<'a> {
    signals: [&'a str; 10],
    outputs: [&'a str; 4],
}

impl<'a> Data<'a> {
    fn parse(line: &'a str) -> Result<Self> {
        let (signals, outputs) = line
            .split_once(" | ")
            .ok_or_else(|| eyre!("invalid input: {}", line))?;

        let mut signals_iter = signals.split_whitespace();
        let mut signals = [""; 10];
        for sig in &mut signals {
            *sig = signals_iter
                .next()
                .ok_or_else(|| eyre!("invalid input - not enough signals: {}", line))?;
        }

        let mut outputs_iter = outputs.split_whitespace();
        let mut outputs = [""; 4];
        for out in &mut outputs {
            *out = outputs_iter
                .next()
                .ok_or_else(|| eyre!("invalid input - not enough outputs: {}", line))?;
        }

        Ok(Self { outputs, signals })
    }
}

fn part1(data: &[Data]) -> u16 {
    data.iter()
        .flat_map(|d| d.outputs.into_iter())
        .map(|d| matches!(d.len(), 2 | 4 | 3 | 7) as u16)
        .sum()
}

fn decode_signals(data: Data) -> u64 {
    // Just look for the 1, 4, and 7 cases.
    let mut one_chars = BTreeSet::new();
    let mut four_chars = BTreeSet::new();
    let mut seven_chars = BTreeSet::new();

    for output in data.signals.into_iter().chain(data.outputs) {
        match output.len() {
            2 => {
                if one_chars.is_empty() {
                    one_chars.extend(output.chars());
                }
            }
            4 => {
                if four_chars.is_empty() {
                    four_chars.extend(output.chars());
                }
            }
            3 => {
                if seven_chars.is_empty() {
                    seven_chars.extend(output.chars());
                }
            }
            _ => {}
        }
    }

    // We know that anything found in the one-chars can only be the C or F segments.
    // We'll filter them out from the four_chars and seven_chars.
    for char in &one_chars {
        four_chars.remove(char);
        seven_chars.remove(char);
    }
    let cf_pos = BTreeSet::from_iter(one_chars.iter().copied());

    // The one segment left for the seven_chars will fix where the A segment is.
    let a_seg = *seven_chars.iter().next().unwrap();

    // After filtering, we know that what's left in four_chars must be the B and D
    // segments.
    let bd_pos = BTreeSet::from_iter(four_chars.iter().copied());

    // We can find the G segment by looking at the signals for the 9 character.
    // It's the one with 6 lit segments, but only a single unknown.
    let mut one_chars_iter = one_chars.iter();
    let (&one_a, &one_b) = one_chars_iter.next().zip(one_chars_iter.next()).unwrap();
    let mut four_chars_iter = four_chars.iter();
    let (&four_a, &four_b) = four_chars_iter.next().zip(four_chars_iter.next()).unwrap();
    let mut zero_chars = BTreeSet::new();
    let mut nine_chars = BTreeSet::new();
    let mut six_chars = BTreeSet::new();

    for d in data
        .signals
        .into_iter()
        .chain(data.outputs)
        .filter(|d| d.len() == 6)
    {
        if !d.contains(four_a) || !d.contains(four_b) {
            zero_chars.extend(d.chars());
        } else if d.contains(one_a) && d.contains(one_b) {
            // The 9 char.
            nine_chars.extend(d.chars());
        } else {
            // The 6 char.
            six_chars.extend(d.chars());
        }
    }

    // We can find out which one G is by filtering out all known characters from
    // nine_chars.
    let mut known_segments: BTreeSet<_> = cf_pos
        .iter()
        .chain(&bd_pos)
        .chain(std::iter::once(&a_seg))
        .copied()
        .collect();

    nine_chars.retain(|c| !known_segments.contains(c));
    assert_eq!(nine_chars.len(), 1);

    let g_seg = *nine_chars.iter().next().unwrap();
    // We know where G is, add it to the known segments and filter the 6 character
    // for the E segment.
    known_segments.insert(g_seg);
    let six_chars2 = six_chars.clone();
    six_chars.retain(|c| !known_segments.contains(c));
    assert_eq!(six_chars.len(), 1);

    let e_seg = *six_chars.iter().next().unwrap();

    // We've now fixed the A, E, and G segments.
    // We can fix the B and D segments by looking at which one in the zero_char
    // *isn't* set.
    let (b_seg, d_seg) = if zero_chars.contains(&four_a) {
        (four_a, four_b)
    } else {
        (four_b, four_a)
    };

    // Now fixed the A, B, D, E, and G segments.
    // The 6 character only has F, not C. Check for one_chars in there.
    let (c_seg, f_seg) = if six_chars2.contains(&one_a) {
        (one_b, one_a)
    } else {
        (one_a, one_b)
    };

    let char_map = [
        // 0
        BTreeSet::from_iter([a_seg, b_seg, c_seg, e_seg, f_seg, g_seg]),
        // 1
        BTreeSet::from_iter([c_seg, f_seg]),
        // 2
        BTreeSet::from_iter([a_seg, c_seg, d_seg, e_seg, g_seg]),
        // 3
        BTreeSet::from_iter([a_seg, c_seg, d_seg, f_seg, g_seg]),
        // 4
        BTreeSet::from_iter([b_seg, c_seg, d_seg, f_seg]),
        // 5
        BTreeSet::from_iter([a_seg, b_seg, d_seg, f_seg, g_seg]),
        // 6
        BTreeSet::from_iter([a_seg, b_seg, d_seg, e_seg, f_seg, g_seg]),
        // 7
        BTreeSet::from_iter([a_seg, c_seg, f_seg]),
        // 8
        BTreeSet::from_iter([a_seg, b_seg, c_seg, d_seg, e_seg, f_seg, g_seg]),
        // 9
        BTreeSet::from_iter([a_seg, b_seg, c_seg, d_seg, f_seg, g_seg]),
    ];

    // dbg!(a_seg, b_seg, c_seg, d_seg, e_seg, f_seg, g_seg);

    let mut sum = 0;
    let mut segment_map = BTreeSet::new();

    for output in data.outputs {
        sum *= 10;
        segment_map.clear();
        segment_map.extend(output.chars());

        for (map, n) in char_map.iter().zip(0..) {
            if map != &segment_map {
                continue;
            }
            sum += n;
        }
    }

    sum
}

fn part2(data: &[Data]) -> u64 {
    data.iter().map(|d| decode_signals(*d)).sum()
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(8).example(Example::Part1, 1).open().unwrap();

        let data: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Data::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(26, part1(&data));
    }

    #[test]
    fn part2_test1() {
        let input = aoc_lib::input(8).example(Example::Part2, 1).open().unwrap();

        let data: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Data::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(5353, part2(&data));
    }

    #[test]
    fn part2_test2() {
        let input = aoc_lib::input(8).example(Example::Part1, 1).open().unwrap();

        let data: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Data::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(61229, part2(&data));
    }
}
