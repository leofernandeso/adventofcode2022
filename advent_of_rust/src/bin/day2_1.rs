use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use phf::phf_map;

// phf_map takes the map and parses it into a match statement
static WINS_AGAINST: phf::Map<char, char> = phf_map! {
    'A' => 'Z', 'B' => 'X', 'C' => 'Y',
    'X' => 'C', 'Y' => 'A', 'Z' => 'B'
};

static PLAY_SCORES: phf::Map<char, u32> = phf_map! {
    'A' => 1, 'B' => 2, 'C' => 3,
    'X' => 1, 'Y' => 2, 'Z' => 3,
};

// &mut borrows the value and allows it to be modified  (like C++ references)
// & borrows the value but does not allow it to be modified (like C++ const references)
// when using pass-by-value syntax in rust, variables expect ones which have primitive types are moved. ownership is transferred

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = parse_arguments();
    let lines = read_lines(input_path)?;
    
    let mut total_player_score: u32 = 0;
    let mut total_opponent_score: u32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut split_line = line.split_whitespace();
            let opponent_choice: char = parse_split_into_choice_character(&mut split_line);
            let player_choice: char = parse_split_into_choice_character(&mut split_line);
            let (player_score, opponent_score) = get_play_outcome(player_choice, opponent_choice);
            total_player_score += player_score;
            total_opponent_score += opponent_score;
        }
    }

    println!("{}", total_player_score);
    println!("{}", total_opponent_score);

    Ok(())
}

fn get_play_outcome(player_choice: char, opponent_choice: char) -> (u32, u32) {
    let mut player_score: u32 = PLAY_SCORES.get(&player_choice).unwrap().clone();
    let mut opponent_score: u32 = PLAY_SCORES.get(&opponent_choice).unwrap().clone();
    if WINS_AGAINST.get(&player_choice).unwrap().clone() == opponent_choice {
        player_score = player_score + 6;
    } else if WINS_AGAINST.get(&opponent_choice).unwrap().clone() == player_choice {
        opponent_score += 6;
    } else {
        player_score += 3;
        opponent_score += 3;
    }
    (player_score, opponent_score)
}

fn parse_split_into_choice_character(split_line: &mut std::str::SplitWhitespace) -> char {
    match split_line.next() {
        Some(player_choice) => player_choice.parse::<char>().unwrap(),
        None => panic!("Cannot parse player choice as single character"),
    }
}

fn read_lines(filename: String) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file_result = File::open(filename);
    match file_result {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found. Please specify a valid file path",
        )),
    }
}

fn parse_arguments() -> String {
    if let Some(input_path) = std::env::args().nth(1) {
        input_path
    } else {
        eprintln!("ERROR: Provide the input path!!!");
        std::process::exit(1);
    }
}
