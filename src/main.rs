use std::io::{self, BufRead};
use sqlite_rust::*;



fn main() {
    logger_builder();

    let mut table= Table::new();

    let stdin= io::stdin();

    input_db();
    for line in stdin.lock().lines() {
        let input= line.unwrap();

        let result_input = match parse_input(&input) {
            Ok(res) => res,
            Err(res) => {
                log::error!("Invalid command {}", res);
                input_db();
                continue;
            },
        };

        match execute_statement(result_input, &mut table) {
          Ok(res) => println!("{}", res),
          Err(res) => {
                log::error!("Invalid command {}", res);
                input_db();
                continue;
            },
        };

        input_db();
    }
}
