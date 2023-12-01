fn main() {
    let input = include_str!("input1.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    let digits = Vec::from(["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]);
    let parts = input.split("\n").collect::<Vec<&str>>();
    let numbers: Vec<i32> = parts
        .iter()
        .map(|x| {
            println!("x: {}", x);
            let mut digits_iter = x
                .chars()
                .filter(|y| digits.contains(&(y.to_string().as_str())))
                .into_iter();
            let first = digits_iter.next();

            let digits_iter = x
                .chars()
                .filter(|y| digits.contains(&(y.to_string().as_str())))
                .into_iter();
            let last = digits_iter.last();

            // Make sure we have both strings
            match (first, last) {
                (Some(first), Some(last)) => {
                    let number = (first.to_string() + last.to_string().as_str()).parse::<i32>();
                    match number {
                        Ok(n) => return n,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
