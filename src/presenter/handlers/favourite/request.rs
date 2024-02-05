use serde::Deserialize;

#[derive(Deserialize, FromForm)]
pub struct FavouriteDeleteParams<'a> {
    pub favourite_uuid: &'a str,
}

#[derive(Deserialize)]
pub struct FavouriteAddBody<'a> {
    pub favourite_uuid: &'a str,
    pub title: &'a str,
}
