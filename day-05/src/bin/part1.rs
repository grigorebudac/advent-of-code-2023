use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let result = process(input);
    dbg!(result);
}

fn process(input: &str) -> u64 {
    let (seeds, maps) = parse_data(input);

    let smallest_location = seeds
        .iter()
        .map(|&seed| find_location_for_seed(seed, &maps, 0))
        .min();

    return smallest_location.unwrap_or(0);
}

fn find_location_for_seed(seed: u64, maps: &Vec<Vec<(u64, u64, u64)>>, map_index: usize) -> u64 {
    let mut location = seed;

    let map = maps.get(map_index).unwrap();

    for instruction in map.iter() {
        let (dest, source, range) = instruction;

        // println!("seed: {}, source: {}, range: {}", seed, *source, range);

        let min = source;
        let max = source + range;

        if seed >= *min && seed <= max {
            let index = seed - *source;
            location = dest + index;
        }
    }

    if map_index < maps.len() - 1 {
        return find_location_for_seed(location, &maps, map_index + 1);
    }

    return location;
}

fn parse_data(input: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let seed_re = Regex::new(r"seeds: ([\d\s]+)").unwrap();
    let map_re = Regex::new(r"(\w+-to-\w+ map):\n((?:\d+ \d+ \d+\n?)+)").unwrap();

    // Parse seeds
    let seeds = seed_re
        .captures(input)
        .and_then(|cap| cap.get(1))
        .map_or(vec![], |m| {
            m.as_str()
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        });

    // Parse maps in order
    let mut maps = Vec::new();
    for cap in map_re.captures_iter(input) {
        let values = cap[2]
            .lines()
            .filter_map(|line| {
                let mut nums = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u64>().ok());

                if let (Some(a), Some(b), Some(c)) = (nums.next(), nums.next(), nums.next()) {
                    Some((a, b, c))
                } else {
                    None
                }
            })
            .collect::<Vec<(u64, u64, u64)>>();

        maps.push(values);
    }

    return (seeds, maps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        ",
        );

        assert_eq!(result, 35)
    }
}
