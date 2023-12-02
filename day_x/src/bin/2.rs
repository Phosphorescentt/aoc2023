fn main() {
    let input = include_str!("input1.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate("");
        assert_eq!(result, i32::MAX);
    }
}
