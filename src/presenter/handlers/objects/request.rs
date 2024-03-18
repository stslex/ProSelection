use serde::Deserialize;

use crate::data::repository::objects::PagingDomainRequest;

#[derive(Deserialize, FromForm)]
pub struct PagingRequest<'a> {
    pub uuid: &'a str,
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

pub async fn map_paging<'a>(
    request_uuid: &'a str,
    request: PagingRequest<'a>,
) -> PagingDomainRequest<'a> {
    PagingDomainRequest {
        user_uuid: request.uuid,
        request_uuid: request_uuid,
        query: request.query,
        page: request.page,
        page_size: request.page_size,
    }
}
