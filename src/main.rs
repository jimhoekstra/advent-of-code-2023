mod day1;
use std::fs::read_to_string;

fn main() {
    print_day1_results();
}

fn print_day1_results() {
    println!("Day one:");
    let input_text = read_to_string("./inputs/day1.txt").unwrap();

    let part_one_result = day1::part_one(input_text.clone());
    println!(" - Part one result is: {}", part_one_result);
    
    let part_two_result = day1::part_two(input_text);
    println!(" - Part two result is: {}\n", part_two_result);
}