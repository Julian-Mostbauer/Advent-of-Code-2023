mod utils;

mod solutions {

    pub mod day1 {
        use crate::utils::general::{digit_word_to_int, get_substring};
        use crate::utils::input::get_file_content;

        pub fn part1() {
            let binding = get_file_content(
            "C:\\Users\\julia\\OneDrive\\Dokumente\\GitHub\\Advent-of-Code-2023\\inputs\\day1.txt",
        );
            let input: Vec<&str> = binding.split_ascii_whitespace().collect();

            let mut sum = 0;
            for line in input {
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
            for line in input {
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
            }
            println!("{}", sum); // 54985
        }
    }
}
fn main() {
    solutions::day1::part2();
}
