use crate::*;

impl_day!(DayOne);

fn replace_string_numbers(string: &String) -> String {
    string 
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

impl DayOne {
    fn solution(&self, part: u8) -> String {
        let mut nums: Vec<String> = vec!();
        for line in &self.file_contents {
            let line: String = if part == 1 { line.to_string() } else { replace_string_numbers(line) };
            let digit_chars: String = line.chars().filter_map(| c | c.is_numeric().then_some(c) ).collect();
            let num: String = match digit_chars.trim().len() {
                0 => "Oh dear. Something has gone wrong.".to_string(),
                _ => {
                    let first_digit = digit_chars.chars().nth(0).unwrap();
                    let last_digit = digit_chars.chars().last().unwrap();
                    format!("{}{}", first_digit, last_digit)
                },
            };
            // println!("line: {}\ndigits: {}\nfinal: {}\n\n", line, digit_chars, num);
            nums.push(num);
        };

        let mut result = 0;
        for num in &nums {
            let num_as_num: u32 = num.parse().unwrap_or_default();
            result += num_as_num;
        };
        
        result.to_string()
    }
}

impl Solve for DayOne {
    fn solve(&self) -> String {
        format!("Part 1: {}\nPart 2: {}", self.solution(1), self.solution(2))
    }
}
