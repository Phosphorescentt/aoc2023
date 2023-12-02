use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    let games = input.split("\n").collect::<Vec<&str>>();
    let mut valid_games_powers: Vec<i32> = Vec::new();
    for game in games {
        if let Some((game_id_str, rounds)) = game.split_once(": ") {
            if let Some((_, id_str)) = game_id_str.split_once(" ") {
                if let Ok(id) = id_str.parse::<i32>() {
                    let largest_cubes = largest_cubes(rounds);
                    let max_red = largest_cubes.get("red").unwrap();
                    let max_green = largest_cubes.get("green").unwrap();
                    let max_blue = largest_cubes.get("blue").unwrap();

                    valid_games_powers.push(max_red * max_green * max_blue);
                }
            }
        } else {
            println!("Unable to split game: {}", game);
        }
    }
    return valid_games_powers.iter().sum();
}

fn largest_cubes(game: &str) -> HashMap<&str, i32> {
    let rounds: Vec<&str> = game.split("; ").collect();
    let mut max_cubes: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for round in rounds.iter() {
        let cubes: Vec<&str> = round.split(", ").collect();
        for cube in cubes.iter() {
            if let Some((number_str, colour)) = cube.split_once(" ") {
                if let Ok(number) = number_str.parse() {
                    if let Some(max_current_colour) = max_cubes.get(colour) {
                        if number > *max_current_colour {
                            max_cubes.insert(colour, number);
                        }
                    }
                }
            }
        }
    }

    return max_cubes;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );
        assert_eq!(result, 2286);
    }
}
