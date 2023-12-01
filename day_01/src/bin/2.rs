const FORWARD_DIGITS: [(&str, &str); 20] = [
    ("0", "0"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
    ("zero", "0"),
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

const BACKWARD_DIGITS: [(&str, &str); 20] = [
    ("0", "0"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
    ("orez", "0"),
    ("eno", "1"),
    ("owt", "2"),
    ("eerht", "3"),
    ("ruof", "4"),
    ("evif", "5"),
    ("xis", "6"),
    ("neves", "7"),
    ("thgie", "8"),
    ("enin", "9"),
];

fn main() {
    let input = include_str!("input2.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    let parts = input.split("\n").collect::<Vec<&str>>();
    let numbers: Vec<i32> = parts
        .iter()
        .map(|x| {
            let first = parse_until_first(x.clone().to_string(), FORWARD_DIGITS);
            let x_reversed: String = x.clone().chars().rev().collect();
            let last = parse_until_first(x_reversed, BACKWARD_DIGITS);

            match (first, last) {
                (Some(first), Some(last)) => {
                    let number = (first.to_string() + last.to_string().as_str()).parse::<i32>();
                    match number {
                        Ok(n) => {
                            println!("n: {}", n);
                            return n;
                        }
                        Err(e) => panic!("{}", e),
                    }
                }
                _ => {
                    println!("Missing either first or last digit in string: {}", x);
                    return 0;
                }
            }
        })
        .collect();

    return numbers.iter().sum();
}

fn parse_until_first(s: String, values: [(&str, &str); 20]) -> Option<String> {
    let first = values
        .map(|v| {
            let idx = s.find(v.0);
            match idx {
                Some(idx) => return (idx, v),
                None => return (usize::MAX, v),
            }
        })
        .into_iter()
        .min_by(|y, z| y.0.cmp(&z.0));

    // println!("first: {:?}", first);
    match first {
        Some(first) => return Some(first.1 .1.to_string()),
        None => return Some("0".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate("two1nine");
        assert_eq!(result, 29);
    }

    #[test]
    fn case2() {
        let result = calculate(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
