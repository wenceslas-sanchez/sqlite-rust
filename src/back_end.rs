use core::fmt;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub id: i8,
    pub username: String,
    pub email: String,
}

#[derive(Clone)]
pub struct Page {
    pub elements: Vec<String>,
    pub num_element: i8,
}

#[derive(Default)]
pub struct Table {
    pub pages: Vec<Page>,
    pub page_size: i8,
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

impl Page {
    pub fn new(element: Option<String>) -> Page {
        let mut elements: Vec<String> = Vec::new();
        let mut num_element= 0;
        if let Some(r) = element {
            elements.push(r);
            num_element += 1;
        }
        Page {
            elements,
            num_element,
        }
    }

    pub fn append(&mut self, element: String) {
        self.num_element += 1;
        self.elements.push(element);
    }
}

impl Table {
    pub fn new(page_size: i8) -> Table {
        Table {
            pages: Vec::new(),
            page_size,
            num_element: 0,
        }
    }

    fn _push_new_page(&mut self, row: String) {
        self.pages.push(
            Page::new(Some(row))
        );
        self.num_element += 1;
    }

    pub fn append(&mut self, row: String) {
        if self.pages.len() == 0 {
            self._push_new_page(row);
            return
        }

        let mut last_page= self.pages.last_mut().unwrap();

        if last_page.num_element < self.page_size {
            last_page.append(row);
            self.num_element += 1;
            return
        } else {
            self._push_new_page(row);
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result_fmt = String::from("Table data:\n");
        let pages = &self.pages;

        for page in pages.iter() {
            for row in page.elements.iter() {
                let deserialized: Row= serde_json::from_str(row).unwrap();
                let row_str = format!("[{}, {}, {}]\n", deserialized.id, deserialized.username, deserialized.email);
                result_fmt.push_str(&row_str);
            }
        }
        write!(f, "{}", result_fmt)
    }
}
