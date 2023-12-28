pub fn part_one(input_text: String) -> u32 {
    let result = input_text.lines().map(|line| {
        let split_string = line.split_once(":")
            .expect("expected a semicolon in the line");
        let game_id_char = split_string.0.chars().last()
            .expect("expected more than zero characters in this string");
        let game_id = format!("{}", game_id_char).parse::<u32>()
            .expect("expected a numeric character"); 

        println!("Game ID: {}", game_id);
        println!("Game: {}", split_string.1);
        game_id
    }).fold(0, |acc, e| acc + e);   

    result
}