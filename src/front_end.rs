use std::io::{self, BufRead, Write};
use std::fmt;

pub enum InputAction {
    Create,
    Delete,
    Insert,
    Select,
    Exit,
}

pub struct InputParsed<'a> {
    pub action: InputAction,
    pub data: Box<Vec<&'a str>>,
}

impl InputParsed<'_> {
    pub fn new(action: InputAction, data: Box<Vec<&str>>) -> InputParsed {
        InputParsed {
            action,
            data
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputError;

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input.")
    }
}


pub fn input_db() {
    print!("db > ");
    io::stdout().flush().unwrap();
}


pub fn parse_input(input: &str) -> Result<InputParsed, InputError> {
    let split= input.split(" ").collect::<Vec<&str>>();

    let input_action= split[0];
    let data= Box::new(split[1..].to_vec());

    match &input_action[..] {
        "create" => Ok(InputParsed::new(InputAction::Create, data)),
        "delete" =>  Ok(InputParsed::new(InputAction::Delete, data)),
        "insert" =>  Ok(InputParsed::new(InputAction::Insert, data)),
        "select" =>  Ok(InputParsed::new(InputAction::Select, data)),
        "exit" =>  Ok(InputParsed::new(InputAction::Exit, data)),
        _ => Err(InputError),
    }
}


pub fn execute_statement(input: InputParsed) {
    match input.action {
        InputAction::Create => println!("Create statement {:?}", input.data),
        InputAction::Delete => println!("Delete statement {:?}", input.data),
        InputAction::Insert => println!("Insert statement {:?}", input.data),
        InputAction::Select => println!("Select statement {:?}", input.data),
        InputAction::Exit => {
            println!("Exit statement");
            std::process::exit(0);
        },
    }
}