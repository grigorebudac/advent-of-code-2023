use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let result = process(&input);
    dbg!(result);
}

fn process(input: &str) -> u32 {
    let mut result = 0;

    let card_values = [
        "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
    ];

    let card_values_map: HashMap<&str, usize> = card_values
        .iter()
        .enumerate()
        .map(|(i, &card)| (card, i + 1))
        .collect();

    let cards = parse_input(&input);

    let mut ranks = Vec::new();

    for card in cards {
        let (value, bid) = card;
        let type_rank = get_rank(&value);
        ranks.push((type_rank, value, bid));
    }

    ranks.sort_by(|a, b| {
        if a.0 == b.0 {
            compare_hands(a.1, b.1, &card_values_map)
        } else {
            a.0.cmp(&b.0)
        }
    });

    for (index, rank) in ranks.iter().enumerate() {
        let (_, _, bid) = rank;

        result += ((index + 1) as u32) * bid.parse::<u32>().unwrap();
    }

    return result;
}

fn compare_hands(
    hand1: &str,
    hand2: &str,
    card_values_map: &HashMap<&str, usize>,
) -> std::cmp::Ordering {
    let mut hand1_iter = hand1.chars();
    let mut hand2_iter = hand2.chars();

    loop {
        if let (Some(card1), Some(card2)) = (hand1_iter.next(), hand2_iter.next()) {
            if card1 == card2 {
                continue;
            }

            let card1_value = card_values_map.get(card1.to_string().as_str()).unwrap();
            let card2_value = card_values_map.get(card2.to_string().as_str()).unwrap();

            if card1_value != card2_value {
                return card2_value.cmp(&card1_value);
            }
        } else {
            // One or both hands have finished
            return std::cmp::Ordering::Equal;
        }
    }
}

fn get_rank(input: &str) -> u8 {
    let mut count_frequencies: HashMap<u8, i32> = HashMap::new();

    for (_, count) in count_cards(input) {
        *count_frequencies.entry(count).or_insert(0) += 1;
    }

    return get_hand_rank(&count_frequencies);
}

fn get_hand_rank(count_frequencies: &HashMap<u8, i32>) -> u8 {
    let five_of_a_kind = count_frequencies.get(&5).is_some();
    let four_of_a_kind = count_frequencies.get(&4).is_some();

    if five_of_a_kind {
        7
    } else if four_of_a_kind {
        6
    } else {
        let has_three_of_a_kind = count_frequencies.get(&3).is_some();
        let has_two_of_a_kind = count_frequencies.get(&2).is_some();
        let full_house = has_three_of_a_kind && has_two_of_a_kind;
        let two_pair = count_frequencies.get(&2) == Some(&2);
        let one_pair = count_frequencies.get(&2) == Some(&1);
        let high_card = count_frequencies.get(&1) == Some(&5);

        if full_house {
            5
        } else if has_three_of_a_kind {
            4
        } else if two_pair {
            3
        } else if one_pair {
            2
        } else if high_card {
            1
        } else {
            0
        }
    }
}

fn count_cards(input: &str) -> Vec<(char, u8)> {
    let mut counts = HashMap::new();

    for c in input.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut count_vec: Vec<(char, u8)> = counts.into_iter().collect();

    count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    return count_vec;
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter_map(|x| {
            let mut whitespaces = x.split_whitespace();

            if let (Some(a), Some(b)) = (whitespaces.next(), whitespaces.next()) {
                Some((a, b))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483",
        );
        assert_eq!(result, 6440)
    }
}
