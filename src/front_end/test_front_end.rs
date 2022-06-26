#[cfg(test)]
mod test_front_end_parse_input {
    use crate::parse_input;
    use crate::front_end::InputAction::{Create, Delete, Exit, Insert, Select};
    use crate::front_end::InputError;

    #[test]
    fn test_parse_input_good() {
        let result= parse_input("insert test1 test2").unwrap();
        assert_eq!(result.action, Insert);
        assert_eq!(&*result.data[0], "test1");
        assert_eq!(&*result.data[1], "test2");
    }

    #[test]
    fn test_parse_input_no_data() {
        let result= parse_input("create").unwrap();
        assert_eq!(result.action, Create);
        assert_eq!(result.data.len(), 0);
    }

    #[test]
    fn test_parse_input_splitted() {
        let result= parse_input("create ").unwrap();
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
            let result= parse_input(x).unwrap();
            assert_eq!(result.action, y);
        }
    }

    #[test]
    fn test_parse_input_unexpected_action() {
        let result= parse_input("tested test1");
        assert!(result.is_err());
        let result= parse_input("tested");
        assert!(result.is_err());
    }

}