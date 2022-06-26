use std::str;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Row {
    pub id: i8,
    pub username: String,
    pub email: String
}

#[derive(Default)]
pub struct Table {
    pub rows: Vec<Box<Row>>,
    pub num_element: i8,
}


impl Row {
    pub fn new(id: i8, username: String, email: String) -> Row{
        Row {
            id,
            username,
            email,
        }
    }
}


impl Table {
    pub fn new() -> Table {
        Table {
            rows: Vec::new(),
            num_element: 0
        }
    }

    pub fn append(&mut self, row: Box<Row>) {
        self.num_element += 1;
        self.rows.push(row);
    }
}
