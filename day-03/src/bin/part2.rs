fn main() {
    let input = include_str!("./input.txt");
    let result = process(input);
    dbg!(result);
}

fn process(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;

    let adjacent_positions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (i, line) in lines.iter().enumerate() {
        for (k, character) in line.chars().enumerate() {
            if character == '*' {
                let mut adjacent_numbers = Vec::new();

                for (di, dk) in &adjacent_positions {
                    let next_i = i as isize + di;
                    let next_k = k as isize + dk;

                    // Boundary checks
                    if next_i < 0
                        || next_i >= lines.len() as isize
                        || next_k < 0
                        || next_k >= line.len() as isize
                    {
                        continue;
                    }

                    let next_char = lines[next_i as usize].chars().nth(next_k as usize).unwrap();

                    if next_char.is_digit(10) {
                        let number = extract_number(&lines, next_i as usize, next_k as usize);

                        if !adjacent_numbers.contains(&number) {
                            adjacent_numbers.push(number);
                        }
                    }
                }

                if adjacent_numbers.len() == 2 {
                    result += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }

    return result;
}

fn extract_number(lines: &Vec<&str>, line_index: usize, char_index: usize) -> i64 {
    let line = lines[line_index];
    let mut number_str = String::new();

    let mut index = char_index;

    // Extract the entire number from left to right
    while index > 0 && line.chars().nth(index - 1).unwrap().is_digit(10) {
        index -= 1;
    }

    while index < line.len() && line.chars().nth(index).unwrap().is_digit(10) {
        number_str.push(line.chars().nth(index).unwrap());
        index += 1;
    }

    number_str.parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    ",
        );
        assert_eq!(result, 467835);
    }

    #[test]
    fn case1() {
        let result = process(
            "
12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56
    ",
        );

        let man_result = 78 * 78 + 12 * 56;
        assert_eq!(result, man_result);
    }
}
