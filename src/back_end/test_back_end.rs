#[cfg(test)]
mod test_back_end_page {
    use crate::back_end::Page;

    #[test]
    fn test_page_constructor() {
        let page = Page::new(None);
        assert_eq!(page.num_element, 0);

        let data = String::from("okok");
        let page = Page::new(Some(data.clone()));
        assert_eq!(page.num_element, 1);
        assert_eq!(page.elements[0], data);
    }

    #[test]
    fn test_page_append() {
        let mut page = Page::new(None);
        let data = String::from("okok");
        page.append(data.clone());
        assert_eq!(page.num_element, 1);
        assert_eq!(page.elements[0], data);
    }
}

#[cfg(test)]
mod test_back_end_table {
    use crate::back_end::Table;

    #[test]
    fn test_table_constructor() {
        let table = Table::new(2);
        assert_eq!(table.page_size, 2);
        assert_eq!(table.num_element, 0);
    }

    #[test]
    fn test_table_append_good() {
        let mut table = Table::new(2);

        let data1 = String::from("okok1");
        let data2 = String::from("okok2");
        let data3 = String::from("okok3");

        table.append(data1.clone());
        assert_eq!(table.num_element, 1);
        table.append(data2.clone());
        assert_eq!(table.num_element, 2);
        table.append(data3.clone());
        assert_eq!(table.num_element, 3);

        assert_eq!(table.pages.len(), 2);
        assert_eq!(table.pages[0].elements[0], data1);
        assert_eq!(table.pages[0].elements[1], data2);
        assert_eq!(table.pages[1].elements[0], data3);
    }

    #[test]
    fn test_table_append_bad() {
        // No limitation
        let mut table = Table::new(0);

        let data1 = String::from("okok1");
        let data2 = String::from("okok2");
        let data3 = String::from("okok3");

        table.append(data1.clone());
        assert_eq!(table.num_element, 1);
        table.append(data2.clone());
        assert_eq!(table.num_element, 2);
        table.append(data3.clone());
        assert_eq!(table.num_element, 3);

        assert_eq!(table.pages.len(), 3);
    }
}
