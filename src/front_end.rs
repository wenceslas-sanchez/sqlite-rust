use std::fmt;
use std::io::{self, BufRead, Write};
use std::str;
#[path = "./back_end.rs"]
mod back_end;
pub use back_end::{Row, Table};
mod test_front_end;

#[derive(PartialEq, Debug)]
pub enum InputAction {
    Create,
    Delete,
    Insert,
    Select,
    Exit,
}

#[derive(PartialEq, Debug)]
pub struct InputParsed {
    pub action: InputAction,
    pub data: Box<Vec<String>>,
}

impl InputParsed {
    pub fn new(action: InputAction, data: Box<Vec<String>>) -> InputParsed {
        InputParsed { action, data }
    }
}

#[derive(Debug)]
pub struct InputError;

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input.")
    }
}

pub fn input_db<W: Write>(mut writer: W) {
    print!("db > ");
    writer.flush().unwrap();
}

pub fn parse_input(input: &str) -> Result<InputParsed, InputError> {
    let split = input.split(' ').collect::<Vec<&str>>();

    let input_action = split[0];
    let data: Box<Vec<String>> = Box::new(split[1..].iter().map(|s| String::from(&**s)).collect());

    match input_action {
        "create" => Ok(InputParsed::new(InputAction::Create, data)),
        "delete" => Ok(InputParsed::new(InputAction::Delete, data)),
        "insert" => Ok(InputParsed::new(InputAction::Insert, data)),
        "select" => Ok(InputParsed::new(InputAction::Select, data)),
        "exit" => Ok(InputParsed::new(InputAction::Exit, data)),
        _ => Err(InputError),
    }
}

pub fn execute_statement(input: InputParsed, table: &mut Table) -> Result<i8, InputError> {
    match input.action {
        InputAction::Create => {
            println!("Create statement table {:?}", input.data[0]);
            Ok(1)
        }
        InputAction::Delete => {
            println!("Delete statement {:?}", input.data);
            Ok(1)
        }
        InputAction::Insert => {
            let result_id = input.data[0].clone().parse::<i8>();
            if result_id.is_err() {
                return Err(InputError);
            };

            let row = Row::new(
                result_id.unwrap(),
                input.data[1].clone(),
                input.data[2].clone(),
            );
            table.append(Box::new(row));

            println!("Insert statement {:?}", input.data);
            Ok(table.num_element)
        }
        InputAction::Select => {
            println!("Select statement {:?}", input.data);
            Ok(1)
        }
        InputAction::Exit => {
            println!("Exit statement");
            std::process::exit(0);
        }
    }
}
