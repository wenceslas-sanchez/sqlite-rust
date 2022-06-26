#[cfg(test)]
mod test_front_end_parse_input {
    use crate::front_end::InputAction::{Create, Delete, Exit, Insert, Select};
    use crate::front_end::InputError;
    use crate::parse_input;

    #[test]
    fn test_parse_input_good() {
        let result = parse_input("insert test1 test2").unwrap();
        assert_eq!(result.action, Insert);
        assert_eq!(&*result.data[0], "test1");
        assert_eq!(&*result.data[1], "test2");
    }

    #[test]
    fn test_parse_input_no_data() {
        let result = parse_input("create").unwrap();
        assert_eq!(result.action, Create);
        assert_eq!(result.data.len(), 0);
    }

    #[test]
    fn test_parse_input_splitted() {
        let result = parse_input("create ").unwrap();
        assert_eq!(result.action, Create);
        assert_eq!(result.data.len(), 1);
        assert_eq!(&*result.data[0], "");
    }

    #[test]
    fn test_parse_input_all_action() {
        for (x, y) in [
            ("insert", Insert),
            ("create", Create),
            ("delete", Delete),
            ("select", Select),
            ("exit", Exit),
        ] {
            let result = parse_input(x).unwrap();
            assert_eq!(result.action, y);
        }
    }

    #[test]
    fn test_parse_input_unexpected_action() {
        let result = parse_input("tested test1");
        assert!(result.is_err());
        let result = parse_input("tested");
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod test_front_end_execute_statement {
    use crate::{execute_statement, InputParsed, parse_input, Table};

    fn generate_input_parser(action: &str, data: &str) -> InputParsed {
        parse_input(&*format!("{} {}", action, data)).unwrap()
    }

    #[test]
    fn test_execute_statement_insert_good() {
        let mut table = Table::new();

        // Only insert statement insert data.
        let input_parsed= generate_input_parser("insert", "15 test3 test4 test5");
        let result= execute_statement(input_parsed, &mut table);
        assert_eq!(result.unwrap(), 1);
        assert_eq!(table.num_element, 1);

        let input_parsed= generate_input_parser("insert", "15 test3 test4 test5");
        let result= execute_statement(input_parsed, &mut table);
        assert_eq!(result.unwrap(), 2);
        assert_eq!(table.num_element, 2);

        let input_parsed= generate_input_parser("insert", "2 test6 test7 ");
        let result= execute_statement(input_parsed, &mut table);
        assert_eq!(result.unwrap(), 3);
        assert_eq!(table.num_element, 3);
    }

    #[test]
    fn test_execute_statement_create_good() {
        let mut table = Table::new();

        // Nothing happens for the moment
        let input_parsed= generate_input_parser("create", "1 test1 test2");
        let result= execute_statement(input_parsed, &mut table);
        assert_eq!(result.unwrap(), 0);
        assert_eq!(table.num_element, 0);
    }

    #[test]
    fn test_execute_statement_select_good() {
        let mut table = Table::new();

        // Nothing happens for the moment
        let input_parsed= generate_input_parser("select", "1 test1 test2");
        let result= execute_statement(input_parsed, &mut table);
        assert_eq!(result.unwrap(), 0);
        assert_eq!(table.num_element, 0);
    }

    #[test]
    fn test_execute_statement_delete_good() {
        let mut table = Table::new();

        // Nothing happens for the moment
        let input_parsed= generate_input_parser("delete", "1 test1 test2");
        let result= execute_statement(input_parsed, &mut table);
        assert_eq!(result.unwrap(), 0);
        assert_eq!(table.num_element, 0);
    }

    #[test]
    fn test_execute_statement_exit_good() {
        let mut table = Table::new();

    }
}