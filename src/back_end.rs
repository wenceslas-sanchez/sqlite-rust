use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Row<'a> {
    pub id: i8,
    pub username: &'a str,
    pub email: &'a str
}

#[derive(Default)]
pub struct Table<'a> {
    pub rows: Vec<Row<'a>>,
    pub num_element: i8,
}

impl<'a> Table<'a> {
    pub fn new() -> Table<'a> {
        Table {
            rows: Vec::new(),
            num_element: 0
        }
    }

    pub fn append(&mut self, row: Row<'a>) {
        self.num_element += 1;
        self.rows.push(row);
    }
}
