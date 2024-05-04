use super::{
    objects::{MatchesDbError, MatchesEntity, MatchesEntityCreate},
    MatchesDatabase,
};
use crate::{
    data::repository::objects::{PagingDomainRequest, PagingDomainResponse},
    schema::matches,
    Conn,
};
use diesel::{
    ExpressionMethods, PgArrayExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods,
};
use uuid::Uuid;

#[async_trait]
impl MatchesDatabase for Conn {
    async fn get_match(&self, id: String) -> Result<MatchesEntity, MatchesDbError> {
        let matches_id = Uuid::parse_str(&id).map_err(|_| MatchesDbError::UuidInvalid)?;
        self.0
            .run(move |db| {
                matches::table
                    .filter(matches::uuid.eq(matches_id))
                    .first::<MatchesEntity>(db)
                    .map_err(|err| {
                        println!("{:?}", err);
                        match err {
                            diesel::result::Error::NotFound => MatchesDbError::MatchesNotFound,
                            _ => MatchesDbError::InternalError,
                        }
                    })
            })
            .await
    }
    async fn add_match(
        &self,
        match_entity: MatchesEntityCreate,
    ) -> Result<MatchesEntity, MatchesDbError> {
        self.0
            .run(move |db| {
                diesel::insert_into(matches::table)
                    .values(&match_entity)
                    .get_result::<MatchesEntity>(db)
            })
            .await
            .map_err(|err| {
                println!("Database add_match error: {:?}", err);
                match err {
                    diesel::result::Error::NotFound => MatchesDbError::MatchesNotCreated,
                    _ => MatchesDbError::InternalError,
                }
            })
    }

    async fn get_matches<'a>(
        &self,
        request: PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<MatchesEntity>, MatchesDbError> {
        let query = request.query.to_owned();
        let uuid = Uuid::parse_str(request.request_uuid).map_err(|e| {
            println!("Database get_matches error: {:?}", e);
            MatchesDbError::UuidInvalid
        })?;
        let page_number = if request.page <= 0 {
            1
        } else {
            request.page - 1
        };
        let limit = request.page_size;
        let offset = page_number * request.page_size;

        self.0
            .run(move |db| {
                let request_uuid = uuid.to_owned();
                let query_request = matches::table
                    .filter(matches::title.like(format!("%{}%", query)))
                    .filter(matches::participants_uuid.contains(vec![request_uuid]));

                let data_request = query_request.to_owned();
                let data = data_request
                    .limit(limit)
                    .offset(offset)
                    .load::<MatchesEntity>(db)?;

                let total_request = query_request.to_owned();
                let total_result = total_request.count().get_result(db)?;

                Ok(PagingDomainResponse {
                    total: total_result,
                    result: data,
                    page: page_number,
                    page_size: request.page_size,
                    has_more: offset < total_result,
                })
            })
            .await
            .map_err(|err: diesel::result::Error| {
                println!("Database get_matches error: {:?}", err);
                MatchesDbError::InternalError
            })
    }

    async fn get_match_count<'a>(&self, user_uuid: &'a str) -> Result<i64, MatchesDbError> {
        let uuid = Uuid::parse_str(user_uuid).map_err(|e| {
            println!("Database parce uuid error: {:?}", e);
            MatchesDbError::UuidInvalid
        })?;
        self.0
            .run(move |db| {
                matches::table
                    .filter(matches::participants_uuid.contains(vec![uuid]))
                    .count()
                    .get_result(db)
            })
            .await
            .map_err(|err| {
                println!("Database get_match_count error: {:?}", err);
                MatchesDbError::InternalError
            })
    }
}
