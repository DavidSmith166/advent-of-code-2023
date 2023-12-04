use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn get_digits_from_line(line: &str) -> (u32, u32) {
    lazy_static! {
        static ref WORD_TO_DIGIT: HashMap<&'static str, u32> = {
            let mut map = HashMap::new();
            map.insert("one", 1);
            map.insert("two", 2);
            map.insert("three", 3);
            map.insert("four", 4);
            map.insert("five", 5);
            map.insert("six", 6);
            map.insert("seven", 7);
            map.insert("eight", 8);
            map.insert("nine", 9);
            map
        };
        static ref DIGIT_SEARCH: Regex =
            Regex::new(r"(?i)(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    }

    let mut digit1 = None;
    let mut digit2 = None;
    let mut next_start = 0;
    while let Some(digit_match) = DIGIT_SEARCH.find_at(line, next_start) {
        next_start = digit_match.start() + 1;
        let text = digit_match.as_str();
        let norm_text = text.to_ascii_lowercase();
        let matched_digit = {
            if WORD_TO_DIGIT.contains_key(norm_text.as_str()) {
                WORD_TO_DIGIT.get(norm_text.as_str()).unwrap().to_owned()
            } else {
                text.chars().next().unwrap().to_digit(10).unwrap()
            }
        };
        if digit1.is_none() {
            digit1 = Some(matched_digit);
        }
        digit2 = Some(matched_digit);
    }
    assert!(digit1.is_some(), "There were no digits in line");
    assert!(digit2.is_some(), "Must be a parse error");
    let digit1 = digit1.unwrap();
    let digit2 = digit2.unwrap();
    (digit1, digit2)
}

fn combine_digits(d1: u32, d2: u32) -> u32 {
    let num_str = format!("{}{}", d1, d2);
    num_str
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("{}{} does not a number make", d1, d2))
}

fn main() {
    let mut sum = 0;
    let document = fs::read_to_string("input.txt").expect("404 File not found");
    for line in document.lines() {
        let (d1, d2) = get_digits_from_line(line);
        let num = combine_digits(d1, d2);
        // println!("Line: {}, d1: {}, d2: {}, num: {}", line, d1, d2, num);
        sum += num;
    }
    println!("sum: {}", sum);
}
