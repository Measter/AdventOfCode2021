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

fn make_map<const N: usize>(segments: &[u8]) -> [u8; N] {
    let mut map = [0; N];
    map[..segments.len()].copy_from_slice(segments);
    map.sort_unstable();
    map
}

fn decode_signals(data: Data) -> u64 {
    // Just look for the 1, 4, and 7 cases.
    let mut one_chars = [0; 2];
    let mut four_chars = [0; 4];
    let mut seven_chars = [0; 3];

    for output in data.signals {
        match output.len() {
            2 if one_chars[0] == 0 => one_chars = make_map(output.as_bytes()),
            4 if four_chars[0] == 0 => four_chars = make_map(output.as_bytes()),
            3 if seven_chars[0] == 0 => seven_chars = make_map(output.as_bytes()),
            _ => {}
        }
    }

    // We know that anything found in the one-chars can only be the C or F segments.
    // We'll filter them out from the four_chars and seven_chars.
    for char in one_chars {
        four_chars.iter_mut().for_each(|fc| {
            if *fc == char {
                *fc = 0
            }
        });
        seven_chars.iter_mut().for_each(|sc| {
            if *sc == char {
                *sc = 0
            }
        });
    }
    four_chars.sort_unstable();
    seven_chars.sort_unstable();

    // The one segment left for the seven_chars will fix where the A segment is.
    let [.., a_seg] = seven_chars;

    // After filtering, we know that what's left in four_chars must be the B and D
    // segments.

    // We can find the G segment by looking at the signals for the 9 character.
    // It's the one with 6 lit segments, but only a single unknown.
    let [one_a, one_b] = one_chars;
    let [.., four_a, four_b] = four_chars;
    let mut zero_chars = [0; 6];
    let mut nine_chars = [0; 6];
    let mut six_chars = [0; 6];

    for d in data
        .signals
        .into_iter()
        .filter(|d| d.len() == 6)
        .map(|d| d.as_bytes())
    {
        if !d.contains(&four_a) || !d.contains(&four_b) {
            zero_chars = make_map(d);
        } else if d.contains(&one_a) && d.contains(&one_b) {
            // The 9 char.
            nine_chars = make_map(d);
        } else {
            // The 6 char.
            six_chars = make_map(d);
        }
    }

    // We can find out which one G is by filtering out all known characters from
    // nine_chars.
    let g_seg = (|| {
        let known_segments = [one_a, one_b, four_a, four_b, a_seg];
        for nc in nine_chars {
            if !known_segments.contains(&nc) {
                return nc;
            }
        }
        panic!("bad nine-char");
    })();

    // We know where G is, add it to the known segments and filter the 6 character
    // for the E segment.
    let e_seg = (|| {
        let known_segments = [one_a, one_b, four_a, four_b, a_seg, g_seg];
        for nc in six_chars {
            if !known_segments.contains(&nc) {
                return nc;
            }
        }
        panic!("bad six-char");
    })();

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
