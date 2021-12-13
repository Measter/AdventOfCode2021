use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult};
use color_eyre::Report;

pub const DAY: Day = Day {
    day: 9,
    name: "Smoke Basin",
    part_1: run_part1,
    part_2: Some(run_part2),
    parse: Some(run_parse),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let map = Map::parse(input);
    b.bench(|| Ok::<_, NoError>(part1(&map)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let map = Map::parse(input);
    b.bench(|| Ok::<_, NoError>(part2(&map)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let map = Map::parse(input);
        Ok::<_, Report>(ParseResult(map))
    })
}

struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = Vec::new();

        for line in input.lines().map(str::as_bytes) {
            width = line.len();
            height += 1;
            tiles.extend(line.iter().map(|t| *t - b'0'));
        }

        Self {
            width,
            height,
            tiles,
        }
    }
}

fn part1(map: &Map) -> u64 {
    let mut risk_level = 0;

    for (y, row) in map.tiles.chunks_exact(map.width).enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            let mut all_higher = true;

            if let Some(y) = y.checked_sub(1) {
                all_higher &= unsafe { *map.tiles.get_unchecked(y * map.width + x) } > tile;
            }
            if let Some(x) = x.checked_sub(1) {
                all_higher &= unsafe { *map.tiles.get_unchecked(y * map.width + x) } > tile;
            }
            if x < (map.width - 1) {
                all_higher &= unsafe { *map.tiles.get_unchecked(y * map.width + x + 1) } > tile;
            }
            if y < (map.height - 1) {
                all_higher &= unsafe { *map.tiles.get_unchecked((y + 1) * map.width + x) } > tile;
            }

            if all_higher {
                risk_level += (tile as u64) + 1;
            }
        }
    }

    risk_level
}

struct Top3Basins {
    basins: [u64; 3],
}

impl Top3Basins {
    fn push(&mut self, mut new_basin: u64) {
        for basin in &mut self.basins {
            if new_basin > *basin {
                std::mem::swap(basin, &mut new_basin);
            }
        }
    }
}

fn part2(map: &Map) -> u64 {
    let mut basin_map: Vec<_> = map.tiles.iter().map(|&height| height == 9).collect();

    let mut basin_sizes = Top3Basins { basins: [0; 3] };
    let mut neighbour_queue = Vec::new();

    for tile_idx in 0..basin_map.len() {
        if basin_map[tile_idx] {
            // This tile has either already been counted, or it was a 9.
            continue;
        }

        basin_map[tile_idx] = true;
        let mut basin_size = 1;

        let (x, y) = (tile_idx % map.width, tile_idx / map.width);
        match y.checked_sub(1) {
            Some(y) if !basin_map[y * map.width + x] => neighbour_queue.push(y * map.width + x),
            _ => {}
        }
        match x.checked_sub(1) {
            Some(x) if !basin_map[y * map.width + x] => neighbour_queue.push(y * map.width + x),
            _ => {}
        }
        if x < (map.width - 1) && !basin_map[y * map.width + x + 1] {
            neighbour_queue.push(y * map.width + x + 1);
        }
        if y < (map.height - 1) && !basin_map[(y + 1) * map.width + x] {
            neighbour_queue.push((y + 1) * map.width + x);
        }

        while let Some(nb_idx) = neighbour_queue.pop() {
            if basin_map[nb_idx] {
                // This tile has already been seen, or was a 9.
                continue;
            }

            basin_size += 1;
            basin_map[nb_idx] = true;

            let (x, y) = (nb_idx % map.width, nb_idx / map.width);

            match y.checked_sub(1) {
                Some(y) if unsafe { !*basin_map.get_unchecked(y * map.width + x) } => {
                    neighbour_queue.push(y * map.width + x)
                }
                _ => {}
            }
            match x.checked_sub(1) {
                Some(x) if unsafe { !*basin_map.get_unchecked(y * map.width + x) } => {
                    neighbour_queue.push(y * map.width + x)
                }
                _ => {}
            }
            if x < (map.width - 1) && unsafe { !*basin_map.get_unchecked(y * map.width + x + 1) } {
                neighbour_queue.push(y * map.width + x + 1);
            }
            if y < (map.height - 1) && unsafe { !*basin_map.get_unchecked((y + 1) * map.width + x) }
            {
                neighbour_queue.push((y + 1) * map.width + x);
            }
        }

        basin_sizes.push(basin_size);
    }

    basin_sizes.basins.into_iter().product()
}

#[cfg(test)]
mod tests_template {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let input = aoc_lib::input(9).example(Example::Part1, 1).open().unwrap();

        let map = Map::parse(&input);

        assert_eq!(15, part1(&map));
    }

    #[test]
    fn part2_test() {
        let input = aoc_lib::input(9).example(Example::Part1, 1).open().unwrap();

        let map = Map::parse(&input);

        assert_eq!(1134, part2(&map));
    }
}
