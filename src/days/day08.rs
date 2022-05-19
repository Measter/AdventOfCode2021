use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 8,
    name: "Seven Segment Search",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

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
    let data: Vec<_> = parse(input).map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<Vec<Data>> {
    let data: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Data::parse)
        .collect::<Result<_, _>>()?;

    Ok(data)
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

fn encode(signal: &str) -> u8 {
    let bytes = signal.as_bytes();
    let bytes = &bytes[..bytes.len().min(7)];

    bytes.iter().map(|b| 1 << (b - b'a')).sum()
}

fn decode_signals(data: Data) -> u64 {
    let mut signals = data.signals.map(encode);
    signals.sort_unstable_by_key(|s| s.count_ones());

    // Just look for the 1, 4, and 7 cases.
    // The 1 case only has 2 bits set, which is the least of any.
    let cf_wires = signals[0];

    // We know that anything found in the one_wires can only be the C or F segments.
    // We'll filter them out from the four_chars and seven_chars.
    let bd_wires = signals[2] & !cf_wires;
    // After filtering, we know that what's left in four_wires must be the B and D
    // segments.

    // The one segment left for the seven_wires will fix where the A segment is.
    let a_seg = signals[1] & !cf_wires;

    // Now we can find out which is the 0, 6, and 9 digits by comparing the
    // overlap with the 1, and 4 digits. Both 6 and 9 have both unknown wires
    // in the 4-digits, while 0 only has one.
    // Similarly, 9 contains both wires from the 1-digit, but 6 is missing one.
    let mut zero_wires = 0;
    let mut nine_wires = 0;
    let mut six_wires = 0;

    for &sig in &signals[6..9] {
        if (sig & bd_wires).count_ones() == 1 {
            // The 0 char.
            zero_wires = sig;
        } else if (sig & cf_wires).count_ones() == 1 {
            // The 6 char.
            six_wires = sig;
        } else {
            // The 9 char.
            nine_wires = sig;
        }
    }

    // We can find out which wire is G by filtering out all known wires from
    // nine_wires.
    let g_seg = nine_wires & !(a_seg | cf_wires | bd_wires);

    // We know where G is, add it to the known segments and filter the 6 character
    // for the E segment.
    let e_seg = six_wires & !(a_seg | g_seg | bd_wires | cf_wires);

    // We've now fixed the A, E, and G segments.
    // We can fix the B and D segments by looking at which one in the zero_char
    // *isn't* set.
    let b_seg = zero_wires & bd_wires;
    let d_seg = bd_wires & !zero_wires;

    // Now fixed the A, B, D, E, and G segments.
    // The 6 character only has F, not C. Check for one_chars in there.
    let f_seg = six_wires & cf_wires;
    let c_seg = cf_wires & !six_wires;

    let digit_map = [
        // 0
        a_seg | b_seg | c_seg | e_seg | f_seg | g_seg,
        // 1
        c_seg | f_seg,
        // 2
        a_seg | c_seg | d_seg | e_seg | g_seg,
        // 3
        a_seg | c_seg | d_seg | f_seg | g_seg,
        // 4
        b_seg | c_seg | d_seg | f_seg,
        // 5
        a_seg | b_seg | d_seg | f_seg | g_seg,
        // 6
        a_seg | b_seg | d_seg | e_seg | f_seg | g_seg,
        // 7
        a_seg | c_seg | f_seg,
        // 8
        a_seg | b_seg | c_seg | d_seg | e_seg | f_seg | g_seg,
        // 9
        a_seg | b_seg | c_seg | d_seg | f_seg | g_seg,
    ];

    let mut sum = 0;
    for output in data.outputs {
        sum *= 10;
        let wires = encode(output);

        for (&map, n) in digit_map.iter().zip(0..) {
            if map == wires {
                sum += n;
            }
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
