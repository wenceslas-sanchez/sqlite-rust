use std::io::{self, BufRead, ErrorKind, Write};
use log;

use sqlite_rust::*;


fn main() {
    logger_builder();

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

        execute_statement(result_input);
        input_db();
    }
}
