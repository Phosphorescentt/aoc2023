use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    let cards = input.lines();

    let mut card_counts: HashMap<i32, i32> = HashMap::new();

    for i in 1..(cards.clone().count() + 1) {
        card_counts.insert(i as i32, 1);
    }

    let total: i32 = cards
        .map(|card| {
            if let Some((card_info, numbers)) = card.split_once(":") {
                let card_id = card_info
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                if let Some((winning_numbers_str, numbers_str)) = numbers.split_once("|") {
                    let winning_numbers = winning_numbers_str
                        .split_whitespace()
                        .collect::<Vec<&str>>();
                    let numbers = numbers_str.split_whitespace().collect::<Vec<&str>>();

                    let no_winning_cards: i32 = winning_numbers
                        .iter()
                        .map(|winning_num| numbers.contains(&winning_num) as i32)
                        .sum();

                    for i in (card_id + 1)..(card_id + 1 + no_winning_cards) {
                        if let Some(future_count) = card_counts.get(&i) {
                            if let Some(current_count) = card_counts.get(&card_id) {
                                card_counts.insert(i, future_count + current_count);
                            } else {
                                card_counts.insert(i, future_count + 1);
                            }
                        } else {
                            card_counts.insert(i, 1);
                        }
                    }
                }
            }
            return 0;
        })
        .sum();

    let total: i32 = card_counts.iter().map(|(_, v)| v).sum();

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
