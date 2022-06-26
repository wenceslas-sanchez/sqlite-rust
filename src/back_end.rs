use core::fmt;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub id: i8,
    pub username: String,
    pub email: String,
}

#[derive(Default)]
pub struct Table {
    pub rows: Vec<Box<Row>>,
    pub num_element: i8,
}

impl Row {
    pub fn new(id: i8, username: String, email: String) -> Row {
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
            num_element: 0,
        }
    }

    pub fn append(&mut self, row: Box<Row>) {
        self.num_element += 1;
        self.rows.push(row);
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result_fmt = String::from("Table data:\n");
        let rows = &self.rows;

        for row in rows.iter() {
            let row_str = format!("[{}, {}, {}]\n", row.id, row.username, row.email);
            result_fmt.push_str(&row_str);
        }
        write!(f, "{}", result_fmt)
    }
}
