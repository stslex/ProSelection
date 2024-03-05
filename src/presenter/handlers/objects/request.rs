use serde::Deserialize;

use crate::data::repository::objects::PagingDomainRequest;

#[derive(Deserialize, FromForm)]
pub struct PagingUuidRequest<'a> {
    pub uuid: &'a str,
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Deserialize, FromForm)]
pub struct PagingRequest<'a> {
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

pub async fn map_paging_uuid<'a>(
    uuid: &'a str,
    request: PagingUuidRequest<'a>,
) -> PagingDomainRequest<'a> {
    PagingDomainRequest {
        user_uuid: request.uuid,
        request_uuid: uuid,
        query: request.query,
        page: request.page,
        page_size: request.page_size,
    }
}
