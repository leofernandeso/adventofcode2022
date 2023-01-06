use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = parse_arguments();
    println!("{}", input_path);    
    let lines = read_lines(input_path)?;

    let biggest_sum = find_max_carbs(lines)?;
    println!("{}", biggest_sum);

    Ok(())
}

fn find_max_carbs(input_file_lines: io::Lines<BufReader<File>>) -> Result<u64, Box<dyn Error>> {
    let mut biggest_sum: u64 = 0;
    let mut current_sum: u64 = 0;
    for line_result in input_file_lines {
        let carbs_amount_string = line_result?;
        if carbs_amount_string.is_empty() {
            if current_sum > biggest_sum {
                biggest_sum = current_sum;
            }
            current_sum = 0;
            continue;
        }
        let carbs_amount = carbs_amount_string.parse::<u64>()?;
        current_sum += carbs_amount;
    }
    Ok(biggest_sum)
}

fn read_lines(filename: String) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file_result = File::open(filename);
    match file_result {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(_) => Err(io::Error::new(io::ErrorKind::NotFound, "File not found. Please specify a valid file path")),
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