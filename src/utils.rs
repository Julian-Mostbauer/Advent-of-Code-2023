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
}
