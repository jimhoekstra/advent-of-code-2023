pub fn part_one(input_text: String) -> u32 {
    let result = input_text.lines().map(|line| {
        let first_digit_index = line.find(char::is_numeric).unwrap();
        let last_digit_index = line.rfind(char::is_numeric).unwrap();

        let first_digit = line.chars().nth(first_digit_index).unwrap();
        let last_digit = line.chars().nth(last_digit_index).unwrap();

        let number = format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap();
        number
    }).fold(0, |acc, e| acc + e);

    result
}

pub fn part_two(input_text: String) -> u32 {
    let replaced_input_text = input_text.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5v")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let result = part_one(replaced_input_text);
    result
}