#[cfg(test)]
mod tests {

    use tokio_test::assert_err;
    use uuid::Uuid;

    use crate::data::database::utils::paging::{correct_page_number, parce_uuid};

    #[test]
    fn test_paging_page_number_negative() {
        let page_number = -45;

        let correct_page_number = correct_page_number(page_number);
        assert_eq!(correct_page_number, 0);
    }

    #[test]
    fn test_paging_page_number_zero() {
        let page_number = 0;

        let correct_page_number = correct_page_number(page_number);
        assert_eq!(correct_page_number, 0);
    }

    #[test]
    fn test_paging_page_number_prozitive() {
        let page_number = 123;

        let correct_page_number = correct_page_number(page_number);
        assert_eq!(correct_page_number, page_number - 1);
    }

    #[test]
    fn test_parse_uuid_correct() {
        let expected_uuid = Uuid::new_v4();
        let uuid = expected_uuid.to_string();

        let parse_uuid = parce_uuid(&uuid);
        assert_eq!(parse_uuid.unwrap(), expected_uuid);
    }

    #[test]
    fn test_parse_uuid_incorrect() {
        let parse_uuid = parce_uuid("incorrect_uuid");
        assert_err!(parse_uuid);
    }
}
