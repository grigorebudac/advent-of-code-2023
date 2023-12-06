fn main() {
    let input = include_str!("./input.txt");
    let result = process(&input);
    dbg!(result);
}

fn process(input: &str) -> u64 {
    let mut result = 1;

    let (time_allowed, best_distance) = parse_data(&input);
    let mut ways_to_win = 0;

    for i in 1..time_allowed {
        let time_remaining = time_allowed - i;
        let distance = i * time_remaining;

        if distance > best_distance {
            ways_to_win += 1;
        }
    }

    result *= ways_to_win;

    return result;
}

fn parse_data(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();

    let time: i64 = lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>()
        .unwrap_or(0);

    let distance: i64 = lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>()
        .unwrap_or(0);

    return (time, distance);
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

        assert_eq!(result, 71503);
    }
}
