fn main() {
    let input = include_str!("./input.txt");
    let result = process(input);
    dbg!(result);
}

fn process(input: &str) -> i32 {
    let mut result = 0;

    let adjacent_positions: [Box<dyn Fn(usize, usize) -> (isize, isize)>; 8] = [
        Box::new(|i, k| (i as isize, k as isize - 1)), // left
        Box::new(|i, k| (i as isize, k as isize + 1)), // right
        Box::new(|i, k| (i as isize - 1, k as isize)), // top
        Box::new(|i, k| (i as isize + 1, k as isize)), // bottom
        Box::new(|i, k| (i as isize - 1, k as isize - 1)), // top-left
        Box::new(|i, k| (i as isize - 1, k as isize + 1)), // top-right
        Box::new(|i, k| (i as isize + 1, k as isize - 1)), // bottom-left
        Box::new(|i, k| (i as isize + 1, k as isize + 1)), // bottom-right
    ];

    let lines: Vec<&str> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let mut acc_number: Option<String> = None;
        let mut has_adjacent = false;

        for (k, character) in line.chars().enumerate() {
            if !character.is_digit(10) {
                if let Some(value) = acc_number {
                    if has_adjacent {
                        result += value.parse::<i32>().unwrap();
                    }

                    acc_number = None;
                    has_adjacent = false;
                }

                continue;
            }

            // accumulate the number
            match acc_number {
                Some(ref mut value) => value.push_str(&character.to_string()),
                None => acc_number = Some(character.to_string()),
            }

            // check adjiacent
            for adjacent_position in &adjacent_positions {
                let (next_line_pos, next_char_pos) = adjacent_position(i, k);

                if let Some(adj_line) = lines.get(next_line_pos as usize) {
                    if let Some(value) = adj_line.chars().nth(next_char_pos as usize) {
                        if value != '.' && !value.is_digit(10) {
                            has_adjacent = true;
                        }
                    }
                }
            }

            // las character
            if k >= line.len() - 1 {
                if let Some(value) = acc_number {
                    if has_adjacent {
                        result += value.parse::<i32>().unwrap();
                    }

                    acc_number = None;
                    has_adjacent = false;
                }
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
        assert_eq!(result, 4361);
    }

    #[test]
    fn case1() {
        let result = process("
    .....180.........230..........................218.....189......415.......................322....507..................206..............111...
    ........*.602.........571-.......................*...*.............199.....$.........181.......*......980....292............................
    ..509.923.=....................+......835*......608.984..............-.801..922.156...*.........533.....$.......*678.......&................
    ...*............273..........307..........393................@..........*.......*...231..................................106.339............
            ");

        let first_line = 180 + 218 + 189 + 507;
        let second_line = 602 + 571 + 199 + 181 + 980 + 292;
        let third_line = 509 + 923 + 835 + 608 + 984 + 801 + 922 + 156 + 533 + 678;
        let fourth_line = 307 + 393 + 231 + 106;

        let man_result = first_line + second_line + third_line + fourth_line;

        assert_eq!(result, man_result);
    }

    #[test]
    fn case2() {
        let result = process("
            ..246....99............316..970...71..$.......606.......@........339...#..#..181.343.693.........542...729=........*..................357...
            ...............781....&.......#.....*.........../.................#..172.761......................*................360...93&................
            ");

        let first_line = 316 + 970 + 71 + 606 + 339 + 542 + 729;
        let second_line = 172 + 761 + 360 + 93;

        let man_result = first_line + second_line;

        assert_eq!(result, man_result);
    }
}
