use std::env;
use std::fs;

struct Input {}
impl Input {
    fn read_line() -> String {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        Self::get_file_content(&buf);
        buf.trim().to_string()
    }

    fn get_file_content(file_name: &str) -> String {
        fs::read_to_string(file_name.trim()).expect("utlf")
    }
}

fn main() {
    part2();
}

fn part1() {
    let binding = Input::get_file_content(
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

fn part2() {
    let binding = Input::get_file_content(
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
                let substring: String = get_substring(line, i, i + lenght).unwrap_or("").to_owned();
                if digit_word_to_int(substring.to_string()).is_some() {
                    ints_in_line.push(digit_word_to_int(substring.to_string()).unwrap());
                }
            }
        }

        sum += ints_in_line.first().unwrap() * 10 + ints_in_line.last().unwrap();
    }
    println!("{}", sum);
}

fn get_substring(s: &str, start: usize, end: usize) -> Result<&str, &'static str> {
    s.get(start..end).ok_or("Invalid indices for substring")
}

fn digit_word_to_int(word: String) -> Option<i32> {
    match word.as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        &_ => None,
    }
}
