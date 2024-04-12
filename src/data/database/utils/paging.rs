use uuid::Uuid;

pub fn correct_page_number(page: i64) -> i64 {
    if page <= 0 {
        0
    } else {
        page - 1
    }
}

pub fn parce_uuid<'a>(uuid: &'a str) -> Result<Uuid, uuid::Error> {
    match Uuid::parse_str(uuid) {
        Ok(uuid) => Ok(uuid),
        Err(err) => {
            eprintln!("Error parsing uuid: {}", err);
            Err(err)
        }
    }
}
