mod day1;
mod day2;
use std::fs::read_to_string;

fn main() {
    print_day1_results();
    print_day2_results();
}

fn print_day1_results() {
    println!("\nDay one:");
    let input_text = read_to_string("./inputs/day1.txt").unwrap();

    let part_one_result = day1::part_one(input_text.clone());
    println!(" - Part one result is: {}", part_one_result);
    
    let part_two_result = day1::part_two(input_text);
    println!(" - Part two result is: {}", part_two_result);
}

fn print_day2_results() {
    println!("\nDay two:");
    let input_text = read_to_string("./inputs/day2.txt").unwrap();

    let part_one_result = day2::part_one(input_text.clone());
    println!(" - Part one result is: {}", part_one_result);

    let part_two_result = day2::part_two(input_text);
    println!(" - Part two result is: {}", part_two_result);
}