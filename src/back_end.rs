#[path = "./back_end/test_back_end.rs"]
mod test_back_end;

use core::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str;
use serde::ser::{SerializeSeq, SerializeStruct};

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub id: i8,
    pub username: String,
    pub email: String,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Page {
    pub elements: Vec<String>,
    pub num_element: i8,
}

#[derive(Default)]
pub struct Table {
    pub pages: Vec<String>,
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
        let mut num_element = 0;
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
        let page= Page::new(Some(row));
        self.pages.push(serde_json::to_string(&page).unwrap());
        self.num_element += 1;
    }

    pub fn append(&mut self, row: String) {
        if self.pages.len() == 0 {
            self._push_new_page(row);
            return;
        }

        let mut last_page = self.pages.last_mut().unwrap();
        let mut last_page_deserialized: Page= serde_json::from_str(last_page).unwrap();

        if last_page_deserialized.num_element < self.page_size {
            last_page_deserialized.append(row);
            let last_page_serialized= serde_json::to_string(&last_page_deserialized).unwrap();

            self.pages.remove(&self.pages.len()-1);
            self.pages.push(last_page_serialized);
            self.num_element += 1;

            return;

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
            let page_deserialized: Page= serde_json::from_str(page).unwrap();
            for row in page_deserialized.elements.iter() {
                let deserialized: Row = serde_json::from_str(row).unwrap();
                let row_str = format!(
                    "[{}, {}, {}]\n",
                    deserialized.id, deserialized.username, deserialized.email
                );
                result_fmt.push_str(&row_str);
            }
        }
        write!(f, "{}", result_fmt)
    }
}
