fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    dbg!(result);
}

fn part2(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        let mut characters = String::new();

        for character in line.chars() {
            characters.push(character);

            let digit = get_digit_from_chars(&characters);

            if digit.is_none() {
                continue;
            }

            if first_digit.is_none() {
                first_digit = digit;
                break;
            }
        }

        characters = String::new();

        for character in line.chars().rev() {
            characters = character.to_string() + &characters;

            let digit = get_digit_from_chars(&characters);

            if digit.is_none() {
                continue;
            }

            if last_digit.is_none() {
                last_digit = digit;
                break;
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let concatenated = first.to_string() + &last.to_string();

            if let Ok(num) = concatenated.parse::<u32>() {
                result += num;
            }
        }
    }

    return result;
}

fn get_digit_from_chars(input: &str) -> Option<String> {
    let mut result: Option<String> = None;

    let digits = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for (word, digit) in digits.iter() {
        if input.contains(word) {
            result = Some(digit.to_string());
            break;
        }

        if input.contains(digit) {
            result = Some(digit.to_string());
            break;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );

        assert_eq!(result, 281);
    }

    #[test]
    fn case1() {
        let result = part2(
            "
two1nine
",
        );

        assert_eq!(result, 29);
    }
    #[test]
    fn case2() {
        let result = part2(
            "
eightwo1nine
",
        );

        assert_eq!(result, 89);
    }

    #[test]
    fn case3() {
        let result = part2(
            "
bctthnksix4onegmgpjbbxqqrmk5hnfvzcnsgh
2sixthreeonenine5ndxdjgkb
sixtfk2qjpq
twohccqqrfour1zjlqq
3nine6
2zrstwopsmjmzdff
89three
",
        );
        let result = 65 + 25 + 62 + 21 + 36 + 2 + 83;
        assert_eq!(result, result);
    }

    #[test]
    fn case4() {
        let result = part2(
            "
7two5
3lnksqznzh
five5four32two
llkbrncncr5nvsmbrzdnine6
nine57
7rdtplhbvddvlkonefqsttj
four6mssqzhgxt
gphnqxhlhzzftghk767
mbcbpjcsnt4six
one9xmhvzklmzffive1kcsixmnsbm2
1dgschj
nine8foursnczninednds
9sevensixrsrgmclkvthkgtxqtwovtlxfrdnllxvsghslh
seven443six8three31
",
        );
        let man_result = 75 + 33 + 52 + 56 + 97 + 71 + 46 + 77 + 46 + 12 + 11 + 99 + 92 + 71;
        assert_eq!(result, man_result);
    }

    #[test]
    fn case5() {
        let result = part2(
            "
eightwo1nineight
",
        );

        assert_eq!(result, 88);
    }
}
