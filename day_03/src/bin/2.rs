use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

fn calculate(input: &str) -> i32 {
    let IDX_TO_RELATIVE_X_Y: HashMap<usize, (i32, i32)> = HashMap::from([
        (0, (-1, -1)),
        (1, (0, -1)),
        (2, (1, -1)),
        (3, (-1, 0)),
        (4, (1, 0)),
        (5, (-1, 1)),
        (6, (0, 1)),
        (7, (1, 1)),
    ]);

    let lines = input.lines().collect::<Vec<&str>>();
    let grid = lines
        .iter()
        .map(|line| {
            let chars = line.chars();
            let mut new_vec: Vec<char> = Vec::new();
            for c in chars {
                new_vec.push(c);
            }

            return new_vec;
        })
        .collect::<Vec<Vec<char>>>();

    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..(grid[0].clone().len()) {
            if grid[y][x] == '*' {
                let neighbours = get_neighbours(x, y, &grid);
                let mut numbers: Vec<Option<i32>> = neighbours
                    .iter()
                    .enumerate()
                    .map(|(i, neighbour)| {
                        if let Some(_) = neighbour {
                            let (num_x, num_y) = IDX_TO_RELATIVE_X_Y.get(&(i as usize)).unwrap();
                            let number = collect_number(
                                (x as i32 + num_x) as usize,
                                (y as i32 + num_y) as usize,
                                &grid,
                            );
                            return number;
                        } else {
                            return None;
                        }
                    })
                    .collect();

                // Remove duplication on top
                if numbers[0] == numbers[1] {
                    numbers[0] = None;
                }
                if numbers[1] == numbers[2] {
                    numbers[1] = None;
                }

                // Remove duplication on bottom
                if numbers[5] == numbers[6] {
                    numbers[5] = None;
                }
                if numbers[6] == numbers[7] {
                    numbers[6] = None;
                }

                let raw_numbers: Vec<i32> = numbers
                    .iter()
                    .filter(|e| {
                        if e.is_some() {
                            if e.unwrap() != 1 {
                                return true;
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    })
                    .map(|e| {
                        if let Some(x) = e {
                            return *x;
                        } else {
                            return 1;
                        }
                    })
                    .collect();

                if raw_numbers.len() == 2 {
                    total += raw_numbers.iter().product::<i32>();
                }
            }
        }
    }

    return total;
}

fn collect_number(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Option<i32> {
    let max_len = grid[0].len() - 1;
    let mut num_string: String = "".to_string();
    let mut building_forwards = true;
    let mut building_backwards = true;

    // Put the first one in!
    if grid[y][x].is_numeric() {
        num_string.push(grid[y][x]);
    } else {
        return None;
    }

    if let Some(row) = grid.get(y) {
        let mut i = 1;
        loop {
            if building_backwards {
                if (x as i32 - i as i32) < 0 {
                    building_backwards = false;
                } else {
                    let prev_char = row[x - i];
                    if prev_char.is_numeric() {
                        num_string.insert(0, prev_char);
                    } else {
                        building_backwards = false;
                    }
                }
            }
            if building_forwards {
                if (x as i32 + i as i32) > max_len as i32 {
                    building_forwards = false;
                } else {
                    let next_char = row[x + i];
                    if next_char.is_numeric() {
                        num_string.push(next_char);
                    } else {
                        building_forwards = false;
                    }
                }
            }

            if !building_backwards & !building_forwards {
                break;
            }

            i += 1;
        }
    }
    return Some(num_string.parse::<i32>().unwrap());
}

fn get_neighbours(x: usize, y: usize, grid: &Vec<Vec<char>>) -> [Option<&char>; 8] {
    // Given an x & y, return an array of 8 items containing the neighbours.
    // Example 1
    // Input: 1 2 3
    //        4 x 5
    //        6 7 8
    // get_neighbours(1, 1): [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8)]
    //
    // Example 2
    // Input: x 1
    //        2 3
    // get_neighbours(0, 0): [None, None, None, None, Some(1), None, Some(2), Some(3)]
    //
    // Example 3
    // Input: 1 2
    //        3 x
    // get_neighbours(1, 1): [Some(1), Some(2), None, Some(3), None, None, None, None]
    let height = grid.len();
    let width = grid[0].len();

    let mut neighbours: [Option<&char>; 8] = [None; 8];

    let top_y = y as i32 - 1;
    let bot_y = y as i32 + 1;
    let top_x = x as i32 - 1;
    let bot_x = x as i32 + 1;

    // top row
    if top_y >= 0 {
        if let Some(top_row) = grid.get(y - 1) {
            if top_x >= 0 {
                neighbours[0] = top_row.get(x - 1);
            }
            neighbours[1] = top_row.get(x);
            if bot_x as usize <= width {
                neighbours[2] = top_row.get(x + 1);
            }
        }
    }

    // middle row
    if let Some(mid_row) = grid.get(y) {
        if top_x >= 0 {
            neighbours[3] = mid_row.get(x - 1);
        }
        if bot_x as usize <= width {
            neighbours[4] = mid_row.get(x + 1);
        }
    }

    // bottom_row
    if bot_y as usize <= height {
        if let Some(bot_row) = grid.get(y + 1) {
            if top_x >= 0 {
                neighbours[5] = bot_row.get(x - 1);
            }
            neighbours[6] = bot_row.get(x);
            if bot_x as usize <= width {
                neighbours[7] = bot_row.get(x + 1);
            }
        }
    }

    return neighbours;
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
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_neighbours() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let lines = input.lines().collect::<Vec<&str>>();
        let grid = lines
            .iter()
            .map(|line| {
                let chars = line.chars();
                let mut new_vec: Vec<char> = Vec::new();
                for c in chars {
                    new_vec.push(c);
                }

                return new_vec;
            })
            .collect::<Vec<Vec<char>>>();

        assert_eq!(
            get_neighbours(0, 0, &grid),
            [
                None,
                None,
                None,
                None,
                Some(&'6'),
                None,
                Some(&'.'),
                Some(&'.')
            ]
        );

        assert_eq!(
            get_neighbours(3, 1, &grid),
            [
                Some(&'7'),
                Some(&'.'),
                Some(&'.'),
                Some(&'.'),
                Some(&'.'),
                Some(&'3'),
                Some(&'5'),
                Some(&'.')
            ]
        )
    }

    #[test]
    fn test_collect_number() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let lines = input.lines().collect::<Vec<&str>>();
        let grid = lines
            .iter()
            .map(|line| {
                let chars = line.chars();
                let mut new_vec: Vec<char> = Vec::new();
                for c in chars {
                    new_vec.push(c);
                }

                return new_vec;
            })
            .collect::<Vec<Vec<char>>>();

        assert_eq!(collect_number(0, 0, &grid), Some(467));
        assert_eq!(collect_number(7, 0, &grid), Some(114));
        assert_eq!(collect_number(7, 2, &grid), Some(633));
    }
}
