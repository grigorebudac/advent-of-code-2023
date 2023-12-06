fn main() {
    let input = include_str!("./input.txt");
    let result = process(&input);
    dbg!(result);
}

fn process(input: &str) -> u32 {
    let races = parse_data(&input);

    let mut result = 1;

    for race in races {
        let (time_allowed, best_distance) = race;
        let mut ways_to_win = 0;

        for i in 1..time_allowed {
            let time_remaining = time_allowed - i;
            let distance = i * time_remaining;

            if distance > best_distance {
                ways_to_win += 1;
            }
        }

        result *= ways_to_win;
    }

    return result;
}

fn parse_data(input: &str) -> Vec<(i32, i32)> {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i32> = lines[0]
        .split_whitespace()
        .skip(1) // Skip the "Time:" part
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<i32> = lines[1]
        .split_whitespace()
        .skip(1) // Skip the "Distance:" part
        .filter_map(|s| s.parse().ok())
        .collect();

    return times.into_iter().zip(distances.into_iter()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            "Time:      7  15   30
Distance:  9  40  200",
        );

        assert_eq!(result, 288);
    }
}
