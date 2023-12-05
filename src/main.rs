mod utils;

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
        use crate::utils::input::get_file_content;

        pub fn part1() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day4.txt",
            );
            let lines: Vec<&str> = input.split('\n').collect();

            let mut points: i128 = 0;

            for line in lines {
                let parts: Vec<&str> = line.split(':').collect();

                let num_left: Vec<&str> = parts
                    .get(1)
                    .unwrap()
                    .split('|')
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .split_ascii_whitespace()
                    .collect();

                let num_right: Vec<&str> = parts
                    .get(1)
                    .unwrap()
                    .split('|')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .split_ascii_whitespace()
                    .collect();

                let mut shared_nums: Vec<&str> = Vec::new();
                let mut shared_count = 0;
                for num in num_right {
                    if num_left.contains(&num) {
                        shared_nums.push(num);
                        shared_count += 1;
                    }
                }

                let mut point = 2i128.pow((shared_count - 1).max(0) as u32);
                if shared_count == 0 {
                    point = 0;
                }
                println!("{:.?}", point);
                points += point;
            }
            println!("{:.?}", points);
        }

        pub fn part2() {
            let input = get_file_content(
                "C:/Users/julia/OneDrive/Dokumente/GitHub/Advent-of-Code-2023/inputs/day4.txt",
            );
            let lines: Vec<&str> = input.split('\n').collect();
        }
    }
}
fn main() {
    solutions::day4::part1();
}
