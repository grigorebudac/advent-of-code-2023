fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    dbg!(result);
}

fn part1(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        for character in line.chars() {
            if (character.is_numeric()) {
                if (first_digit.is_none()) {
                    first_digit = Some(character);
                }

                last_digit = Some(character);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = part1(
            "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        );

        assert_eq!(result, 142);
    }

    #[test]
    fn sample_1() {
        let result = part1(
            "1six7396484
1ninehgqtjprgnpkchxdkctzk
sevenmpsmstdfivebtnjljnlnpjrkhhsninefour9
pppmfmnfourtworxrqrfhbgx8vvxgrjzhvqmztltwo
9eightctkdnnllnine
1eight3shhj8hrglbhxsixvhntf
4rlqzthlhkxvzhcm6
bklcbpdlfctwofivesqvpxjjzlvn35zhlljrfqf
36qv",
        );

        let sum = 14 + 11 + 99 + 88 + 99 + 18 + 46 + 35 + 36;
        assert_eq!(result, sum);
    }
}
