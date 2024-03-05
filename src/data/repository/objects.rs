#[derive(Debug, Clone)]
pub struct PagingDomainRequest<'a> {
    pub user_uuid: &'a str,
    pub request_uuid: &'a str,
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone)]
pub struct PagingDomainResponse<T> {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub has_more: bool,
    pub result: Vec<T>,
}
