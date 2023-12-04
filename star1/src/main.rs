use std::fs;

fn main() {
    let mut sum = 0;
    let document = fs::read_to_string("input.txt").expect("404 File not found");
    for line in document.lines() {
        let mut digit1 = None;
        let mut digit2 = None;
        for char in line.chars() {
            if char.is_digit(10) {
                if digit1.is_none() {
                    digit1 = char.to_digit(10);
                }
                digit2 = char.to_digit(10);
            }
        }
        assert!(digit1.is_some(), "There were no digits in line");
        assert!(digit2.is_some(), "Must be a parse error");
        let digit1 = digit1.unwrap();
        let digit2 = digit2.unwrap();
        let num_str = format!("{}{}", digit1, digit2);
        let num = num_str.parse::<u32>().expect(&format!("{}{} does not a number make", digit1, digit2));
        sum += num;
    }
    println!("sum: {}", sum);
}
