use aoc_lib::{day, Bench, BenchResult, NoError};

day! {
   day 3: "Binary Diagnostics"
   1: run_part1
   2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let nums: Vec<_> = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2))
        .collect::<Result<_, _>>()
        .unwrap();

    b.bench(|| Ok::<_, NoError>(part1::<12>(&nums)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let nums: Vec<_> = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2))
        .collect::<Result<_, _>>()
        .unwrap();

    b.bench(|| Ok::<_, NoError>(part2::<12>(&nums)))
}

fn part1<const N: usize>(nums: &[u32]) -> u32 {
    let mut counts = [0; N];

    for &num in nums {
        for (i, ci) in counts.iter_mut().enumerate() {
            *ci += (num & u32::pow(2, i as u32) != 0) as u32;
        }
    }

    let mut gamma = 0;
    for c in counts.into_iter().rev() {
        gamma <<= 1;
        if c > (nums.len() as u32 - c) {
            gamma |= 1;
        }
    }

    let mask = (usize::pow(2, N as u32) - 1) as u32;
    gamma * (!gamma & mask)
}

fn bit_criteria_search<const N: usize>(nums: &[u32], most_common: bool) -> u32 {
    let mut nums = nums.to_owned();
    let mut bit_mask = u32::pow(2, N as u32);

    while nums.len() > 1 {
        bit_mask >>= 1;
        let count_set = nums.iter().filter(|&&num| (num & bit_mask) != 0).count();
        let count_unset = nums.len() - count_set;

        let keep_ones = if most_common {
            count_set >= count_unset
        } else {
            count_set < count_unset
        };

        nums.retain(|&num| match (keep_ones, num & bit_mask) {
            (true, 0) => false,
            (true, _) => true,
            (false, 0) => true,
            (false, _) => false,
        });

        if nums.len() == 1 {
            break;
        }
    }

    nums[0]
}

fn part2<const N: usize>(nums: &[u32]) -> u32 {
    let oxy_num = bit_criteria_search::<N>(nums, true);
    let co2_num = bit_criteria_search::<N>(nums, false);

    oxy_num * co2_num
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(3).example(Example::Part1, 1).open().unwrap();

        let nums: Vec<_> = data
            .lines()
            .map(|l| u32::from_str_radix(l, 2))
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(198, part1::<5>(&nums));
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(3).example(Example::Part1, 1).open().unwrap();

        let nums: Vec<_> = data
            .lines()
            .map(|l| u32::from_str_radix(l, 2))
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(230, part2::<5>(&nums));
    }
}
