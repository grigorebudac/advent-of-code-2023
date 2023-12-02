struct DataSet {
    values: Vec<i32>,
}

struct OptionData {
    id: i32,
    data_sets: Vec<DataSet>,
}

fn main() {
    let input = include_str!("./input.txt");
    let result = process(input, 12, 13, 14);
    dbg!(result);
}

fn process(input: &str, max_r: i32, max_g: i32, max_b: i32) -> i32 {
    let mut options: Vec<OptionData> = Vec::new();
    let mut result = 0;

    for line in input.lines() {
        let option = get_line_data_sets(line);

        if let Ok(value) = option {
            options.push(value);
        }
    }

    for option in options {
        let mut is_good = true;

        for data_set in option.data_sets {
            let r = data_set.values.get(0).unwrap();
            let g = data_set.values.get(1).unwrap();
            let b = data_set.values.get(2).unwrap();

            if r > &max_r || g > &max_g || b > &max_b {
                is_good = false;
                break;
            }
        }

        if is_good {
            result += option.id;
        }
    }

    return result;
}

fn get_line_data_sets(line: &str) -> Result<OptionData, &str> {
    let mut id = 0;
    let mut data_sets: Vec<DataSet> = vec![];

    let main_parts: Vec<&str> = line.split(':').collect();

    if main_parts.len() != 2 {
        return Err("Line is not valid");
    }

    let game_part = main_parts[0];
    let options_part = main_parts[1];

    let parsed_game_part: Vec<&str> = game_part.split_whitespace().collect();
    id = parsed_game_part[1].parse::<i32>().unwrap();

    for option_part in options_part.split(';') {
        let mut data_set = DataSet { values: vec![0,0,0] };
        
        for part_value in option_part.split(',') {
            let parsed_value: Vec<&str> = part_value.split_whitespace().collect();
            let value = parsed_value[0].parse::<i32>().unwrap();
    
            if part_value.contains("red") {
                data_set.values[0] = value;
            }
    
            if part_value.contains("green") {
                data_set.values[1] = value;
            }
    
            if part_value.contains("blue") {
                data_set.values[2] = value;
            }
        }

        data_sets.push(data_set);
    }

    return Ok(OptionData { id, data_sets });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",12,13,14
        );

        
        assert_eq!(result, 8);
    }


}

