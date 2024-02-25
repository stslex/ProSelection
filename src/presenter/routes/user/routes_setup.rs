use rocket::Build;
use rocket::Rocket;

use crate::presenter::routes::routes_setup::BASE_API_URL;
use crate::presenter::routes::user::routes;
use crate::presenter::routes::user::RoutesUserInitialized;

use super::favourite::RouteFavouriteInitialize;
use super::follower::RouteFollowerInitialize;
use super::matches::RouteMatchesInitialize;

impl RoutesUserInitialized for Rocket<Build> {
    fn mount_user_routes(self) -> Self {
        let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL);

        self.mount_favourite_routes()
            .mount_follower_routes()
            .mount_matches_routes()
            .mount(
                base_url,
                routes![
                    routes::get_user_count,
                    routes::get_current_user,
                    routes::get_user,
                    routes::get_user_by_username,
                    routes::get_user_search,
                    routes::post_follow,
                    routes::delete_follow,
                    routes::get_is_following,
                    routes::get_user_followers,
                    routes::get_user_following,
                ],
            )
    }
}

pub const BASE_USER_URL: &str = "/user";
