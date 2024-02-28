use rocket::{Build, Rocket};

use super::RouteFollowerInitialize;
use crate::presenter::routes::user::follower::routes;
use crate::presenter::routes::{routes_setup::BASE_API_URL, user::routes_setup::BASE_USER_URL};

const BASE_FOLLOWER_URL: &str = "/follow";

impl RouteFollowerInitialize for Rocket<Build> {
    fn mount_follower_routes(self) -> Self {
        let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL + BASE_FOLLOWER_URL);
        self.mount(
            base_url,
            routes![
                routes::post_follow,
                routes::delete_follow,
                routes::get_is_following,
                routes::get_user_followers,
                routes::get_user_following,
            ],
        )
    }
}
