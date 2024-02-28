use rocket::{Build, Rocket};

use super::RouteFavouriteInitialize;
use crate::presenter::routes::{
    routes_setup::BASE_API_URL,
    user::{favourite::routes, routes_setup::BASE_USER_URL},
};

impl RouteFavouriteInitialize for Rocket<Build> {
    fn mount_favourite_routes(self) -> Self {
        let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL + BASE_FAVOURITE_URL);
        self.mount(
            base_url,
            routes![
                routes::post_add_favourite,
                routes::delete_remove_favourite,
                routes::get_is_favourite,
                routes::get_user_favourites,
            ],
        )
    }
}

const BASE_FAVOURITE_URL: &str = "/favourite";
