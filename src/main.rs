use sqlite_rust::*;
use std::io::{self, BufRead};

fn main() {
    let writer = io::stdout();
    logger_builder();

    let mut table = Table::new(5);

    let stdin = io::stdin();

    input_db(&writer);
    for line in stdin.lock().lines() {
        let input = line.unwrap();

        let result_input = match parse_input(&input) {
            Ok(res) => res,
            Err(res) => {
                log::error!("Invalid command {}", res);
                input_db(&writer);
                continue;
            }
        };

        match execute_statement(result_input, &mut table) {
            Ok(res) => println!("{}", res),
            Err(res) => {
                log::error!("Invalid command {}", res);
                input_db(&writer);
                continue;
            }
        };

        input_db(&writer);
    }
}
