use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let result = process(input);
    dbg!(result);
}

fn process(input: &str) -> u32 {
    let mut result: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (first_column, second_column) = get_card_numbers(line);
        let winning_numbers = first_column.intersection(&second_column);

        let count = winning_numbers.count();

        if count > 0 {
            result += 2_u32.pow((count - 1) as u32);
        }
    }

    return result;
}

fn get_card_numbers(input: &str) -> (HashSet<i32>, HashSet<i32>) {
    let parts: Vec<&str> = input.split('|').collect();

    return (
        get_numbers_from_str(&parts[0]),
        get_numbers_from_str(&parts[1]),
    );
}

fn get_numbers_from_str(input: &str) -> HashSet<i32> {
    return input
        .split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );

        assert_eq!(result, 13);
    }
}
