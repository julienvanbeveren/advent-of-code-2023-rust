use std::fs;

fn part1() {
    let input_file = fs::read_to_string("./src/input.txt").expect("Unable to read file"); 
    let lines = input_file.lines();
    let mut sum_number = 0;
    for line in lines {
        let mut start = '0';
        let mut end = '0';
        let char_iter = line.chars();
        for letter in char_iter {
            if String::from(letter).parse::<i8>().is_ok() {
                if start == '0' {
                    start = char::from(letter);
                }
                end = char::from(letter);
            }
        }
        let final_char = format!("{}{}", start, end);
        let final_number = final_char.parse::<i32>().unwrap();
        sum_number += final_number;
    }
    println!("Sum of all numbers: {}", sum_number);
}

fn replace_last_occurrence(s: &str, find: &str, replace_with: &str) -> String {
    if let Some(pos) = s.rfind(find) {
        let (start, end) = s.split_at(pos);
        format!("{}{}{}", start, replace_with, &end[find.len()..])
    } else {
        s.to_string()
    }
}

fn str_to_nums(word: &String) -> String {
    let num_words = vec!["_", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut smallest_index = 100000;
    let mut largest_index = 0;
    let mut first_num = "";
    let mut last_num = "";
    let mut final_str = String::from(word);
    for number in num_words.iter() {
        let res = word.find(number);
        if res.is_some() {
            if res.unwrap() < smallest_index {
                smallest_index = res.unwrap();
                first_num = *number;    
            }
            if res.unwrap() + String::from(*number).chars().count() > largest_index {
                largest_index = res.unwrap() + String::from(*number).chars().count();
                last_num = *number;
            }
        }
    }
    if first_num != "" {
        final_str = final_str.replacen(first_num, String::from(num_words.iter().position(|&r| r == first_num).unwrap().to_string()).as_str(), 1);
    }
    if last_num != "" {
        final_str = replace_last_occurrence(&final_str, &last_num, String::from(num_words.iter().position(|&r| r == last_num).unwrap().to_string()).as_str());
    }
    String::from(final_str)
}

fn part2() {
    let input_file = fs::read_to_string("./src/input.txt").expect("Unable to read file"); 
    let lines = input_file.lines();
    let mut sum_number = 0;
    for line in lines {
        let clean = str_to_nums(&String::from(line));
        let mut start = '0';
        let mut end = '0';
        let char_iter = clean.chars();
        for letter in char_iter {
            if String::from(letter).parse::<i8>().is_ok() {
                if start == '0' {
                    start = char::from(letter);
                }
                end = char::from(letter);
            }
        }
        let final_char = format!("{}{}", start, end);
        let final_number = final_char.parse::<i32>().unwrap();
        sum_number += final_number;
    }
    println!("Sum of all numbers: {}", sum_number);
}

fn main() {
    part1();
    part2();
    println!("{}", str_to_nums(&String::from("4nineeightseven2")));
}
