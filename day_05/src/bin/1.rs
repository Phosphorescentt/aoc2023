fn main() {
    let input = include_str!("input.txt");
    let result = calculate(input);
    println!("result: {}", result);
}

#[derive(Clone, Copy, Debug)]
struct MapItem {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

#[derive(Clone, Debug)]
struct Map {
    map_items: Vec<MapItem>,
}

impl Map {
    fn new(input: &Vec<&str>) -> Self {
        let mut map_iter = input.iter();
        let _ = map_iter.next();

        let mut map_items: Vec<MapItem> = Vec::new();
        for row in map_iter {
            let numbers: Vec<i64> = row
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            map_items.push(MapItem {
                destination_range_start: numbers[0],
                source_range_start: numbers[1],
                range_length: numbers[2],
            })
        }

        return Map { map_items };
    }

    fn traverse(&self, input: i64) -> i64 {
        for map_item in &self.map_items {
            if ((map_item.source_range_start)
                ..(map_item.source_range_start + map_item.range_length))
                .contains(&input)
            {
                return input - map_item.source_range_start + map_item.destination_range_start;
            }
        }
        return input;
    }
}

#[derive(Clone, Debug)]
struct Maps {
    maps: Vec<Map>,
}

impl Maps {
    fn new<'a, I>(input_iter: I) -> Maps
    where
        I: Iterator<Item = &'a &'a str>,
    {
        let mut paragraphs: Vec<Vec<&str>> = Vec::new();
        let mut temp_lines: Vec<&str> = Vec::new();
        for item in input_iter {
            if item != &"" {
                temp_lines.push(item);
            } else {
                paragraphs.push(temp_lines.clone());
                temp_lines = Vec::new();
            }
        }
        paragraphs.push(temp_lines.clone());

        return Maps {
            maps: paragraphs.iter().map(|x| Map::new(x)).collect::<Vec<Map>>(),
        };
    }

    fn traverse(maps: Maps, seeds: Vec<i64>) -> Vec<i64> {
        let locations = seeds
            .iter()
            .map(|seed| {
                let mut current_seed_location: i64 = seed.clone();
                for map in &maps.maps {
                    current_seed_location = Map::traverse(&map, current_seed_location)
                }
                return current_seed_location;
            })
            .collect::<Vec<i64>>();
        return locations;
    }
}

fn calculate(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut lines_iter = lines.iter();
    let mut seeds: Vec<i64> = Vec::new();
    if let Some(seeds_line) = lines_iter.next() {
        if let Some((_, seeds_str)) = seeds_line.split_once(":") {
            seeds = seeds_str
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
        }
    }
    // remove that one blank line lol
    lines_iter.next();

    let maps = Maps::new(lines_iter);
    let locations: Vec<i64> = Maps::traverse(maps, seeds);
    return *locations.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let result = calculate(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
        );
        assert_eq!(result, 35);
        // assert!(false);
    }

    #[test]
    fn test_map_traverse() {
        let map = Map {
            map_items: vec![MapItem {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            }],
        };

        assert_eq!(Map::traverse(&map, 40), 40);
        assert_eq!(Map::traverse(&map, 99), 51);

        let map = Map {
            map_items: vec![MapItem {
                destination_range_start: 50,
                source_range_start: 10,
                range_length: 100,
            }],
        };

        assert_eq!(Map::traverse(&map, 100), 140);
    }

    #[test]
    fn test_maps_traverse() {
        let maps = Maps {
            maps: vec![
                Map {
                    map_items: vec![MapItem {
                        destination_range_start: 50,
                        source_range_start: 98,
                        range_length: 2,
                    }],
                },
                Map {
                    map_items: vec![MapItem {
                        destination_range_start: 10,
                        source_range_start: 50,
                        range_length: 2,
                    }],
                },
            ],
        };

        assert_eq!(Maps::traverse(maps, vec![99]), vec![11]);
    }
}
