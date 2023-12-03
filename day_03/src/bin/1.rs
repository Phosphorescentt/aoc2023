const PART_IDENTIFIERS: [char; 10] = ['@', '*', '$', '#', '&', '/', '%', '=', '+', '-'];

fn main() {
    let input = include_str!("input.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    // let mut rows = input.split("\n").collect::<Vec<&str>>();
    let rows = input.lines().collect::<Vec<&str>>();
    let mut total = 0;

    for (i, row) in rows.iter().enumerate() {
        let number_infos = get_numbers_info(row);
        for (number_str, start_x, length) in number_infos.iter() {
            if is_engine_part(*start_x as i32, i as i32, *length as i32, &rows) {
                let number: i32 = number_str.parse().unwrap();
                total += number;
            }
        }
    }

    return total;
}

fn get_numbers(s: &str) -> Vec<(String, usize, usize)> {
    // Get strings for numbers in row and return a vec of those strings,
    // their start indices, and their length.
    let mut number_strs: Vec<String> = Vec::new();
    let cleaned_s = s.replace(&PART_IDENTIFIERS[..], ".");
    let parts = cleaned_s.split(".");
    for part in parts {
        if let Ok(number) = part.parse::<i32>() {
            number_strs.push(number.to_string());
        }
    }

    let number_infos = number_strs
        .iter()
        .map(|number_string| {
            return (
                number_string.to_owned(),
                s.find(number_string).unwrap(),
                number_string.len(),
            );
        })
        .collect();

    return number_infos;
}

fn get_numbers_info(s: &str) -> Vec<(String, usize, usize)> {
    // Get strings for numbers in row and return a vec of those strings,
    // their start indices, and their length.
    let mut number_infos: Vec<(String, usize, usize)> = Vec::new();
    let mut building = false;
    let mut current_number = "".to_string();
    for (i, c) in s.chars().enumerate() {
        if c.is_numeric() {
            if !building {
                building = true;
            }
            current_number.push_str(c.to_string().as_str());
        } else {
            if building {
                building = false;
                number_infos.push((
                    current_number.clone(),
                    i - current_number.len(),
                    current_number.len(),
                ));
                current_number = "".to_string();
            }
        }
    }

    if building {
        number_infos.push((
            current_number.clone(),
            s.len() - current_number.len(),
            current_number.len(),
        ));
    }

    return number_infos;
}

fn is_identifier(c: &char) -> bool {
    return PART_IDENTIFIERS.contains(c);
}

fn is_engine_part(start_x: i32, start_y: i32, number_length: i32, full_input: &Vec<&str>) -> bool {
    // return true if engine part, false otherwise

    let height = full_input.len();
    let length = full_input.get(0).unwrap().len();

    // check start
    if start_x != 0 {
        // check top left
        if start_y > 0 {
            let row = full_input.get((start_y - 1) as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x - 1) as usize).unwrap()) {
                return true;
            }
        }
        // check middle left
        {
            let row = full_input.get(start_y as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x - 1) as usize).unwrap()) {
                return true;
            }
        }
        // check bottom left
        if (start_y as usize) < height - 1 {
            let row = full_input.get((start_y + 1) as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x - 1) as usize).unwrap()) {
                return true;
            }
        }
    }
    // check end
    if (start_x as usize) < length - (number_length as usize) - 1 {
        // check top right
        if start_y > 0 {
            let row = full_input.get((start_y - 1) as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x + number_length) as usize).unwrap()) {
                return true;
            }
        }
        // check middle right
        {
            let row = full_input.get(start_y as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x + number_length) as usize).unwrap()) {
                return true;
            }
        }
        // check bottom right
        if start_y as usize != height - 1 {
            let row = full_input.get((start_y + 1) as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x + number_length) as usize).unwrap()) {
                return true;
            }
        }
    }

    // check middle
    for i in 0..number_length {
        // check above
        if start_y != 0 {
            let row = full_input.get((start_y - 1) as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x + i) as usize).unwrap()) {
                return true;
            }
        }

        // check below
        if start_y as usize != height - 1 {
            let row = full_input.get((start_y + 1) as usize).unwrap().to_string();
            if is_identifier(&row.chars().nth((start_x + i) as usize).unwrap()) {
                return true;
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }

    #[test]
    fn case1_simplified() {
        let result = calculate("467..114..\n..b*......");
        assert_eq!(result, 467);
    }

    #[test]
    fn test_get_numbers_single_number() {
        let numbers_results = get_numbers_info("..1234......");
        assert_eq!(numbers_results, vec![("1234".to_string(), 2, 4)]);
    }

    #[test]
    fn test_get_numbers_multiple_numbers() {
        let numbers_results = get_numbers_info("..1234...56...");
        assert_eq!(
            numbers_results,
            vec![("1234".to_string(), 2, 4), ("56".to_string(), 9, 2)]
        );
    }

    #[test]
    fn test_is_identifier() {
        let c = '*';
        assert_eq!(is_identifier(&c), true);
    }

    #[test]
    fn test_is_engine_part() {
        let rows = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .split("\n");
        let number_infos: Vec<Vec<(String, usize, usize)>> =
            rows.map(|row| get_numbers(row)).collect();
        assert_eq!(
            number_infos,
            vec![
                vec![("467".to_string(), 0, 3), ("114".to_string(), 5, 3)],
                vec![],
                vec![("35".to_string(), 2, 2), ("633".to_string(), 6, 3)],
                vec![],
                vec![("617".to_string(), 0, 3)],
                vec![("58".to_string(), 7, 2)],
                vec![("592".to_string(), 2, 3)],
                vec![("755".to_string(), 6, 3)],
                vec![],
                vec![("664".to_string(), 1, 3), ("598".to_string(), 5, 3)]
            ]
        )
    }
}
