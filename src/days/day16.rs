use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use bitreader::BitReader;
use color_eyre::eyre::{eyre, Report, Result};

// 12:53
// 14:04
// 14:32

pub const DAY: Day = Day {
    day: 16,
    name: "Packet Decoder",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input);
    let mut reader = BitReader::new(&data);
    let packet = Packet::parse(&mut reader).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&packet)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input);
    let mut reader = BitReader::new(&data);
    let packet = Packet::parse(&mut reader).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&packet)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input);
        let mut reader = BitReader::new(&data);
        Ok::<_, Report>(ParseResult(Packet::parse(&mut reader)?))
    })
}

#[derive(Debug, Clone)]
pub struct ArrChunks<'a, T, const N: usize>(&'a [T]);
impl<'a, T, const N: usize> Iterator for ArrChunks<'a, T, N> {
    type Item = &'a [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.get(..N)?;
        self.0 = self.0.get(N..)?;
        next.try_into().ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() / N;
        (len, Some(len))
    }
}

impl<'a, T, const N: usize> ExactSizeIterator for ArrChunks<'a, T, N> {}

impl<'a, T, const N: usize> ArrChunks<'a, T, N> {
    pub fn new(ts: &'a [T]) -> Self {
        assert!(N > 0);
        Self(ts)
    }

    pub fn remaining(&self) -> &'a [T] {
        self.0
    }
}

fn parse(input: &str) -> Vec<u8> {
    let mut data = Vec::new();

    let mut chunks = ArrChunks::new(input.trim().as_bytes());
    for &[hi, lo] in &mut chunks {
        let hi = (hi as char).to_digit(16).expect("bad input");
        let lo = (lo as char).to_digit(16).expect("bad input");

        data.push(((hi << 4) + lo) as u8);
    }

    if let [hi] = chunks.remaining() {
        let hi = (*hi as char).to_digit(16).expect("bad input");
        data.push((hi << 4) as u8);
    }

    data
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketKind {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl PacketKind {
    fn parse(reader: &mut BitReader) -> Result<Self> {
        let packet_type = reader.read_u8(3)?;
        match packet_type {
            // Literal
            4 => {
                let mut num: u64 = 0;
                loop {
                    let is_final = !reader.read_bool().unwrap();
                    num <<= 4;
                    num |= reader.read_u64(4).unwrap();

                    if is_final {
                        break;
                    }
                }
                Ok(Self::Literal(num))
            }
            0..=3 | 5..=7 => {
                let length_type = reader.read_bool()?;
                let sub_packets = match length_type {
                    true => {
                        let num_subpackets = reader.read_u16(11)?;
                        let mut sub_packets = Vec::new();
                        for _ in 0..num_subpackets {
                            sub_packets.push(Packet::parse(reader)?);
                        }

                        sub_packets
                    }
                    false => {
                        let total_length = reader.read_u64(15)?;
                        let start_position = reader.position();

                        let mut sub_packets = Vec::new();
                        while (reader.position() - start_position) < total_length {
                            sub_packets.push(Packet::parse(reader)?);
                        }

                        sub_packets
                    }
                };

                match packet_type {
                    0 => Ok(Self::Sum(sub_packets)),
                    1 => Ok(Self::Product(sub_packets)),
                    2 => Ok(Self::Minimum(sub_packets)),
                    3 => Ok(Self::Maximum(sub_packets)),
                    5 => Ok(Self::GreaterThan(sub_packets)),
                    6 => Ok(Self::LessThan(sub_packets)),
                    7 => Ok(Self::EqualTo(sub_packets)),
                    _ => unreachable!(),
                }
            }
            _ => return Err(eyre!("Unknown packet type: {}", packet_type)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    kind: PacketKind,
}

impl Packet {
    fn parse(reader: &mut BitReader) -> Result<Self> {
        let version = reader.read_u8(3)?;
        let kind = PacketKind::parse(reader)?;
        Ok(Self { version, kind })
    }
}

fn part1(packet: &Packet) -> u64 {
    use PacketKind::*;
    match &packet.kind {
        Literal(_) => packet.version as u64,
        Sum(p) | Product(p) | Minimum(p) | Maximum(p) | GreaterThan(p) | LessThan(p)
        | EqualTo(p) => p
            .iter()
            .fold(packet.version as u64, |acc, p| acc + part1(p)),
    }
}

fn part2(packet: &Packet) -> u64 {
    match &packet.kind {
        PacketKind::Literal(v) => *v,
        PacketKind::Sum(sp) => sp.iter().map(part2).sum(),
        PacketKind::Product(sp) => sp.iter().map(part2).product(),
        PacketKind::Minimum(sp) => sp.iter().map(part2).min().unwrap(),
        PacketKind::Maximum(sp) => sp.iter().map(part2).max().unwrap(),
        PacketKind::GreaterThan(sp) => {
            let b_val = part2(&sp[1]);
            let a_val = part2(&sp[0]);

            (a_val > b_val) as u64
        }
        PacketKind::LessThan(sp) => {
            let b_val = part2(&sp[1]);
            let a_val = part2(&sp[0]);

            (a_val < b_val) as u64
        }
        PacketKind::EqualTo(sp) => {
            let b_val = part2(&sp[1]);
            let a_val = part2(&sp[0]);

            (a_val == b_val) as u64
        }
    }
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::{input, Example};
    use bitreader::BitReader;

    #[test]
    fn parse_test_1() {
        let input = aoc_lib::input(16)
            .example(Example::Parse, 1)
            .open()
            .unwrap();

        let data = parse(&input);
        let mut reader = BitReader::new(&data);

        let expected = Packet {
            version: 6,
            kind: PacketKind::Literal(2021),
        };

        let actual = Packet::parse(&mut reader).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_test_2() {
        let input = aoc_lib::input(16)
            .example(Example::Parse, 2)
            .open()
            .unwrap();

        let data = parse(&input);
        let mut reader = BitReader::new(&data);

        let expected = Packet {
            version: 1,
            kind: PacketKind::LessThan(vec![
                Packet {
                    version: 6,
                    kind: PacketKind::Literal(10),
                },
                Packet {
                    version: 2,
                    kind: PacketKind::Literal(20),
                },
            ]),
        };

        let actual = Packet::parse(&mut reader).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_test_3() {
        let input = aoc_lib::input(16)
            .example(Example::Parse, 3)
            .open()
            .unwrap();

        let data = parse(&input);
        let mut reader = BitReader::new(&data);

        let expected = Packet {
            version: 7,
            kind: PacketKind::Maximum(vec![
                Packet {
                    version: 2,
                    kind: PacketKind::Literal(1),
                },
                Packet {
                    version: 4,
                    kind: PacketKind::Literal(2),
                },
                Packet {
                    version: 1,
                    kind: PacketKind::Literal(3),
                },
            ]),
        };

        let actual = Packet::parse(&mut reader).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test() {
        let input = input(16).example(Example::Part1, 1).open().unwrap();

        for (idx, line) in input.lines().map(str::trim).enumerate() {
            let (packet, expected_value) = line.split_once(" - ").unwrap();
            let expected_value: u64 = expected_value.parse().unwrap();

            let data = parse(packet);
            let mut reader = BitReader::new(&data);
            let packet = Packet::parse(&mut reader).unwrap();

            let actual_value = part1(&packet);

            assert_eq!(expected_value, actual_value, "{}", idx);
        }
    }

    #[test]
    fn part2_test() {
        let input = input(16).example(Example::Part2, 1).open().unwrap();

        for (idx, line) in input.lines().map(str::trim).enumerate() {
            let (packet, expected_value) = line.split_once(" - ").unwrap();
            let expected_value: u64 = expected_value.parse().unwrap();

            let data = parse(packet);
            let mut reader = BitReader::new(&data);
            let packet = Packet::parse(&mut reader).unwrap();

            let actual_value = part2(&packet);

            assert_eq!(expected_value, actual_value, "{}", idx);
        }
    }
}
