use regex::Regex;

pub fn part_one(input_text: String) -> u32 {
    sum_of_valid_game_ids(input_text, 14, 12, 13)
}

pub fn part_two(input_text: String) -> u32 {
    power_of_required_cubes(input_text)
}

fn sum_of_valid_game_ids(list_of_games: String, blue_limit: u32, red_limit: u32, green_limit: u32) -> u32 {
    list_of_games.lines().map(|line| {
        let split_string = line.split_once(":")
            .expect("expected a semicolon in the line");
        let game_id_chars = split_string.0.split_once(" ")
            .expect("expected a space separating the word game from the ID");

        let game_id = format!("{}", game_id_chars.1).parse::<u32>()
            .expect("expected a numeric character"); 

        if required_blue_cubes(split_string.1) <= blue_limit &&
           required_green_cubes(split_string.1) <= green_limit &&
           required_red_cubes(split_string.1) <= red_limit
        {
            return game_id
        }
        
        0
    }).fold(0, |acc, e| acc + e)
}

fn power_of_required_cubes(list_of_games: String) -> u32 {
    list_of_games.lines().map(|line| {
        required_blue_cubes(line) *
        required_green_cubes(line) *
        required_red_cubes(line)
    }).fold(0, |acc, e| acc + e)
}

fn required_blue_cubes(game_line: &str) -> u32 {
    let regex_str = "(?<number>[0-9]+ blue)";
    required_cubes_for_color(game_line, regex_str)
}

fn required_red_cubes(game_line: &str) -> u32 {
    let regex_str = "(?<number>[0-9]+ red)";
    required_cubes_for_color(game_line, regex_str)
}

fn required_green_cubes(game_line: &str) -> u32 {
    let regex_str = "(?<number>[0-9]+ green)";
    required_cubes_for_color(game_line, regex_str)
}

fn required_cubes_for_color(game_line: &str, regex_str: &str) -> u32 {
    let re = Regex::new(regex_str).unwrap();

    let result = re.captures_iter(game_line).map(|caps| {
        let number_as_string = caps.name("number").unwrap().as_str()
            .split_once(" ").unwrap().0;
        let number_as_int = format!("{}", number_as_string).parse::<u32>()
            .expect("expected a numeric character");

        number_as_int
    }).max().unwrap();

    result
}