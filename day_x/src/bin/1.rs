fn main() {
    let input = include_str!("input1.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> String {
    return "day 1 part 1".to_string();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
