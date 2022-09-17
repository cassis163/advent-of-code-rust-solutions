use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt");

    match file {
        Ok(found_file) => {
            let reader = BufReader::new(found_file);

            let mut last_value: Option<i32> = None;
            let mut count: i32 = 0;
            for line in reader.lines() {
                match line {
                    Ok(line_str) => {
                        let current_value_result = line_str.parse::<i32>();
                        match current_value_result {
                            Ok(current_value) => {
                                match last_value {
                                    Some(last) => {
                                        if current_value > last {
                                            count += 1;
                                        }
                                    }
                                    None => (),
                                }

                                last_value = Some(current_value);
                            }
                            Err(e) => panic!("{}", e),
                        }
                    }
                    Err(e) => panic!("{}", e),
                }
            }

            println!("{}", count);
        }
        Err(e) => panic!("{}", e),
    }
}
