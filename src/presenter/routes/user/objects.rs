use serde::Deserialize;

#[derive(Deserialize, FromForm)]
pub struct UserSearchParams<'a> {
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Deserialize, FromForm)]
pub struct UserPagingSearchParams<'a> {
    pub uuid: &'a str,
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}
