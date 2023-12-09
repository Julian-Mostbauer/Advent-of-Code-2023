pub(crate) mod input {
    use std::fs;
    pub fn read_line() -> String {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }

    pub fn get_file_content(file_name: &str) -> String {
        fs::read_to_string(file_name.trim()).unwrap()
    }
}

pub mod general {
    pub fn get_substring(s: &str, start: usize, end: usize) -> Result<&str, &'static str> {
        s.get(start..end).ok_or("Invalid indices for substring")
    }

    pub fn digit_word_to_int(word: String) -> Option<i32> {
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

    pub fn matches_to_points(matches: i128) -> i128 {
        let mut point = 2i128.pow((matches - 1).max(0) as u32);
        if matches == 0 {
            point = 0;
        }
        point
    }

    pub fn get_matches_line(line: &str) -> i128 {
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

        shared_count
    }
}
