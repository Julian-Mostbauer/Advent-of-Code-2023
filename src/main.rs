mod utils;

#[allow(dead_code)]
mod solutions {

    pub mod day1 {
        use crate::utils::general::{digit_word_to_int, get_substring};
        use crate::utils::input::get_file_content;

        pub fn part1() {
            let binding = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day1.txt",
            );
            let input: Vec<&str> = binding.split_ascii_whitespace().collect();

            let mut sum = 0;
            for line in &input {
                let mut ints_in_line: Vec<i32> = Vec::new();
                for c in line.chars() {
                    if c.to_string().parse::<i32>().is_ok() {
                        ints_in_line.push(c.to_string().parse::<i32>().unwrap())
                    }
                }

                sum += ints_in_line.first().unwrap() * 10 + ints_in_line.last().unwrap();
            }
            println!("{}", sum); //55130
        }

        pub fn part2() {
            let binding = get_file_content(
            "C:\\Users\\julia\\OneDrive\\Dokumente\\GitHub\\Advent-of-Code-2023\\inputs\\day1.txt",
        );
            let input: Vec<&str> = binding.split_ascii_whitespace().collect();

            let mut sum = 0;
            input.iter().for_each(|&line| {
                let mut ints_in_line: Vec<i32> = Vec::new();

                for i in 0..line.len() {
                    if line
                        .chars()
                        .nth(i)
                        .unwrap()
                        .to_string()
                        .parse::<i32>()
                        .is_ok()
                    {
                        ints_in_line.push(
                            line.chars()
                                .nth(i)
                                .unwrap()
                                .to_string()
                                .parse::<i32>()
                                .unwrap(),
                        )
                    }
                    for lenght in 3..=5 {
                        let substring: String =
                            get_substring(line, i, i + lenght).unwrap_or("").to_owned();

                        if digit_word_to_int(substring.to_string()).is_some() {
                            ints_in_line.push(digit_word_to_int(substring.to_string()).unwrap());
                        }
                    }
                }

                sum += ints_in_line.first().unwrap() * 10 + ints_in_line.last().unwrap();
            });
            println!("{}", sum); // 54985
        }
    }

    pub mod day2 {
        //12 red cubes, 13 green cubes, and 14 blue cubes
        use crate::utils::input::get_file_content;
        use itertools::Itertools;

        pub fn part1() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day2.txt",
            );

            let mut impossible_games: Vec<i32> = Vec::new();

            let games: Vec<&str> = input.split('\n').collect();

            for game in games {
                let game_info: Vec<&str> = game.split(':').collect();

                let id = game_info
                    .first()
                    .unwrap()
                    .split_ascii_whitespace()
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap_or(&"0")
                    .parse::<i32>()
                    .unwrap_or(0);

                let reveals: Vec<&str> = game
                    .split(':')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .split(';')
                    .collect();

                for reveal in reveals {
                    let dice_info: Vec<&str> = reveal.split(',').collect();

                    for info in dice_info {
                        let parts: Vec<&str> = info.split(' ').collect();

                        let num_part = parts.get(1).unwrap_or(&"0");
                        let color_part = parts.last().unwrap_or(&"red");

                        match *color_part {
                            "red" => {
                                if num_part.parse::<i32>().unwrap() > 12 {
                                    impossible_games.push(id);
                                    println!(
                                        "ID: {} Number: {} Dice: {} ",
                                        id, num_part, color_part
                                    );
                                }
                            }
                            "green" => {
                                if num_part.parse::<i32>().unwrap() > 13 {
                                    impossible_games.push(id);
                                    println!(
                                        "ID: {} Number: {} Dice: {} ",
                                        id, num_part, color_part
                                    );
                                }
                            }
                            "blue" => {
                                if num_part.parse::<i32>().unwrap() > 14 {
                                    impossible_games.push(id);
                                    println!(
                                        "ID: {} Number: {} Dice: {} ",
                                        id, num_part, color_part
                                    );
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            impossible_games = impossible_games.into_iter().unique().collect();

            println!("{:.?}", impossible_games);

            let sum: i32 = (1..=100).sum::<i32>() - impossible_games.iter().sum::<i32>();

            println!("{}", sum);
        }

        pub fn part2() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day2.txt",
            );

            let mut power = 0;

            let games: Vec<&str> = input.split('\n').collect();

            for game in games {
                let game_info: Vec<&str> = game.split(':').collect();

                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                let reveals: Vec<&str> = game
                    .split(':')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .split(';')
                    .collect();

                for reveal in reveals {
                    let dice_info: Vec<&str> = reveal.split(',').collect();

                    for info in dice_info {
                        let parts: Vec<&str> = info.split(' ').collect();

                        let num_part = parts.get(1).unwrap_or(&"0");
                        let color_part = parts.last().unwrap_or(&"red");

                        match color_part.trim() {
                            "red" => {
                                max_red = max_red.max(num_part.parse::<i32>().unwrap());
                            }
                            "green" => {
                                max_green = max_green.max(num_part.parse::<i32>().unwrap());
                            }
                            "blue" => {
                                max_blue = max_blue.max(num_part.parse::<i32>().unwrap());
                            }
                            _ => (),
                        }
                    }
                }

                power += max_blue * max_green * max_red;
            } // 69110

            println!("{}", power);
        }
    }

    pub mod day3 {
        use itertools::Itertools;

        use crate::utils::input::get_file_content;
    }

    pub mod day4 {
        use crate::utils::general::*;
        use crate::utils::input::get_file_content;

        pub fn part1() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day4.txt",
            );
            let lines: Vec<&str> = input.split('\n').collect();

            let mut points: i128 = 0;

            for line in lines {
                let matches = get_matches_line(line);

                matches_to_points(matches);
            }
            println!("{:.?}", points);
        }
    }

    pub mod day8 {
        use crate::utils::input::get_file_content;

        #[derive(Debug, Clone)]
        struct LinkedPath {
            value: String,
            next_left: String,
            next_right: String,
        }
        impl LinkedPath {
            fn next_left(&self) -> String {
                let next_left = &self.next_left;
                next_left.to_string().trim().to_string()
            }
            fn next_right(&self) -> String {
                let next_right = &self.next_right;
                next_right.to_string().trim().to_string()
            }
            fn from_string(input: String) -> LinkedPath {
                let value = input
                    .split('=')
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_string()
                    .trim()
                    .to_string();

                let next_left = input
                    .split('=')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .split(',')
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_string()
                    .split_off(2)
                    .trim()
                    .to_string();

                let mut next_right = input
                    .split('=')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .split(',')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .to_string()
                    .trim()
                    .to_string();

                next_right.pop();

                LinkedPath {
                    value,
                    next_left,
                    next_right,
                }
            }

            fn find_with_value(map: &[LinkedPath], search_value: &str) -> Option<LinkedPath> {
                for path in map {
                    let found_value = path.value.to_string();
                    if found_value == search_value {
                        return Some(path.clone());
                    }
                }
                None
            }
        }

        pub fn part1() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day8.txt",
            );

            let lines = input.split('\n').collect::<Vec<&str>>();
            let lr_instructions: Vec<char> =
                (*lines.first().unwrap().trim().chars().collect::<Vec<char>>()).to_vec();

            let map: Vec<LinkedPath> = Vec::from(&lines[1..])
                .iter()
                .map(|map_element| LinkedPath::from_string(map_element.to_string()))
                .collect();

            let mut counter: usize = 0;
            let mut current = map.first().unwrap().clone();
            while current.value != "ZZZ" {
                for instruction in &lr_instructions {
                    current = match instruction {
                        'L' => LinkedPath::find_with_value(&map, &current.next_left()).unwrap(),
                        'R' => LinkedPath::find_with_value(&map, &current.next_right()).unwrap(),
                        _ => panic!(),
                    };

                    counter += 1;

                    if counter % 100000 == 0 {
                        println!("{:.?} - {}", current, counter);
                    }
                }
            }

            println!("{:.?}", counter);
        }

        pub fn part2_optimised() {}

        pub fn part2() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day8.txt",
            );

            let lines = input.split('\n').collect::<Vec<&str>>();
            let lr_instructions: Vec<char> =
                (*lines.first().unwrap().trim().chars().collect::<Vec<char>>()).to_vec();

            let map: Vec<LinkedPath> = lines[1..]
                .iter()
                .map(|map_element| LinkedPath::from_string(map_element.to_string()))
                .collect();

            let mut counter: usize = 0;
            let mut current_nodes: Vec<LinkedPath> = map
                .iter()
                .filter(|s| s.value.ends_with('A'))
                .cloned()
                .collect::<Vec<LinkedPath>>();

            let total_nodes = map.len();
            let mut z_nodes = current_nodes
                .iter()
                .filter(|s| s.value.ends_with('Z'))
                .count();

            while z_nodes < total_nodes {
                for instruction in &lr_instructions {
                    let mut new_current_nodes = Vec::new();
                    for current in &current_nodes {
                        let next_current = match instruction {
                            'L' => LinkedPath::find_with_value(&map, &current.next_left()).unwrap(),
                            'R' => {
                                LinkedPath::find_with_value(&map, &current.next_right()).unwrap()
                            }

                            _ => panic!(),
                        };
                        counter += 1;
                        new_current_nodes.push(next_current);

                        if counter % 100_000 == 0 {
                            println!("{counter}");
                        }
                    }
                    current_nodes = new_current_nodes;
                }
                z_nodes = current_nodes
                    .iter()
                    .filter(|s| s.value.ends_with('Z'))
                    .count();
            }
        }
    }

    pub mod day9 {
        use itertools::Itertools;

        use crate::utils::general::to_lines;
        use crate::utils::input::get_file_content;

        pub fn part1() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day9.txt",
            );

            let lines = to_lines(input);
            let mut sum = 0;
            for line in &lines {
                let mut differences: Vec<Vec<i32>> = vec![to_num_vec(line)];

                let mut new_differences = get_differences(to_num_vec(line));
                while !new_differences.iter().all(|n| *n == 0) {
                    differences.push(new_differences.clone());
                    new_differences = get_differences(new_differences);
                }
                differences.push(new_differences.clone());

                let mut new_num = 0;

                differences.iter().for_each(|difference| {
                    new_num += difference.last().unwrap();
                });

                sum += new_num;
            }
            println!("{sum}");
        }

        pub fn part2() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day9.txt",
            );

            let lines = to_lines(input);
            let mut sum = 0;

            for line in &lines {
                let mut differences: Vec<Vec<i32>> = vec![to_num_vec(line)];

                let mut new_differences = get_differences(to_num_vec(line));
                while !new_differences.iter().all(|n| *n == 0) {
                    differences.push(new_differences.clone());
                    new_differences = get_differences(new_differences);
                }
                differences.push(new_differences.clone());

                let mut new_num = 0;
                let rev_differences: Vec<Vec<i32>> = differences.into_iter().rev().collect_vec();

                for i in 0..(rev_differences.len() - 1) {
                    new_num = rev_differences[i + 1].first().unwrap() - new_num;
                }

                sum += new_num;
                println!("{new_num}");
            }
            println!("{sum}");
        }

        pub fn get_differences(v: Vec<i32>) -> Vec<i32> {
            let mut result = Vec::new();

            for i in 0..(v.len() - 1) {
                result.push(v[i + 1] - v[i]);
            }

            result
        }

        pub fn to_num_vec(input: &str) -> Vec<i32> {
            input
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        }
    }
}
fn main() {
    solutions::day9::part2();
}
