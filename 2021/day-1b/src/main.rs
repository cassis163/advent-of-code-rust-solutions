use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    let input_file_result = File::open("input.txt");
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(error) => panic!("{}", error),
    };

    let mut count: i32 = 0;
    let mut last_value_option: Option<i32> = None;
    let reader = BufReader::new(input_file);
    for line_result in reader.lines() {
        let current_value_result = match line_result {
            Ok(line) => line.parse::<i32>(),
            Err(e) => panic!("{}", e),
        };

        let current_value = match current_value_result {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        };

        match last_value_option {
            Some(last_value) => {
                if current_value > last_value {
                    count += 1;
                }
            }
            None => (),
        }

        last_value_option = Some(current_value);
    }

    println!("{}", count);
}
