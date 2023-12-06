fn main() {
    let input = include_str!("input.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let times = lines
        .iter()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let distances = lines
        .iter()
        .nth(1)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let ways_of_winning: i32 = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| {
            (1..=*time)
                .map(|i| (time - i) * i)
                .filter(|s| *s > distance)
                .count() as i32
        })
        .product();

    return ways_of_winning;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate(
            "Time:      7  15   30
Distance:  9  40  200
",
        );
        assert_eq!(result, 288);
    }
}
