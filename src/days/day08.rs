use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use nom::AsBytes;

pub const DAY: Day = Day {
    day: 8,
    name: "Seven Segment Search",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: Vec::new(),
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

fn make_map(segments: &[u8]) -> [u8; 7] {
    let mut map = [0; 7];
    map[..segments.len()].copy_from_slice(segments);
    map.sort_unstable();
    map
}

fn decode_signals(mut data: Data) -> u64 {
    data.signals.sort_unstable_by_key(|s| s.len());
    // dbg!(data.signals);
    // panic!();

    // Just look for the 1, 4, and 7 cases.
    let one_chars: [u8; 2] = data.signals[0].as_bytes().try_into().unwrap();
    let mut four_chars: [u8; 4] = data.signals[2].as_bytes().try_into().unwrap();

    // We know that anything found in the one-chars can only be the C or F segments.
    // We'll filter them out from the four_chars and seven_chars.
    for char in one_chars {
        four_chars.iter_mut().for_each(|fc| {
            if *fc == char {
                *fc = 0
            }
        });
    }
    four_chars.sort_unstable();

    // The one segment left for the seven_chars will fix where the A segment is.
    // let [.., a_seg] = seven_chars;
    let a_seg = {
        let mut seg = 0;
        for sc in data.signals[1].as_bytes() {
            if !one_chars.contains(sc) {
                seg = *sc;
            }
        }
        seg
    };

    // After filtering, we know that what's left in four_chars must be the B and D
    // segments.

    // We can find the G segment by looking at the signals for the 9 character.
    // It's the one with 6 lit segments, but only a single unknown.
    let [one_a, one_b] = one_chars;
    let [.., four_a, four_b] = four_chars;
    let mut zero_chars = [0; 6];
    let mut nine_chars = [0; 6];
    let mut six_chars = [0; 6];

    for d in data.signals[6..9].iter().map(|d| d.as_bytes()) {
        let chars: [u8; 6] = d.try_into().unwrap();
        if !chars.contains(&four_a) || !chars.contains(&four_b) {
            // The 0 char.
            zero_chars = chars;
        } else if chars.contains(&one_a) && chars.contains(&one_b) {
            // The 9 char.
            nine_chars = chars;
        } else {
            // The 6 char.
            six_chars = chars;
        }
    }

    // We can find out which one G is by filtering out all known characters from
    // nine_chars.
    let g_seg = {
        let known_segments = [one_a, one_b, four_a, four_b, a_seg];
        let mut seg = 0;
        for nc in nine_chars {
            if !known_segments.contains(&nc) {
                seg = nc;
            }
        }
        seg
    };

    // We know where G is, add it to the known segments and filter the 6 character
    // for the E segment.
    let e_seg = {
        let known_segments = [one_a, one_b, four_a, four_b, a_seg, g_seg];
        let mut seg = 0;
        for nc in six_chars {
            if !known_segments.contains(&nc) {
                seg = nc;
            }
        }
        seg
    };

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
    let (c_seg, f_seg) = if six_chars.contains(&one_a) {
        (one_b, one_a)
    } else {
        (one_a, one_b)
    };

    let char_map: [[u8; 7]; 10] = [
        // 0
        make_map(&[a_seg, b_seg, c_seg, e_seg, f_seg, g_seg]),
        // 1
        make_map(&[c_seg, f_seg]),
        // 2
        make_map(&[a_seg, c_seg, d_seg, e_seg, g_seg]),
        // 3
        make_map(&[a_seg, c_seg, d_seg, f_seg, g_seg]),
        // 4
        make_map(&[b_seg, c_seg, d_seg, f_seg]),
        // 5
        make_map(&[a_seg, b_seg, d_seg, f_seg, g_seg]),
        // 6
        make_map(&[a_seg, b_seg, d_seg, e_seg, f_seg, g_seg]),
        // 7
        make_map(&[a_seg, c_seg, f_seg]),
        // 8
        make_map(&[a_seg, b_seg, c_seg, d_seg, e_seg, f_seg, g_seg]),
        // 9
        make_map(&[a_seg, b_seg, c_seg, d_seg, f_seg, g_seg]),
    ];

    let mut sum = 0;
    for output in data.outputs {
        sum *= 10;
        let segment_map: [u8; 7] = make_map(output.as_bytes());

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
