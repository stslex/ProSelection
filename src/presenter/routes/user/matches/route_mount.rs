use rocket::{Build, Rocket};

use super::RouteMatchesInitialize;
use crate::presenter::routes::user::matches::routes;
use crate::presenter::routes::{routes_setup::BASE_API_URL, user::routes_setup::BASE_USER_URL};

const BASE_MATCH_URL: &str = "/match";

impl RouteMatchesInitialize for Rocket<Build> {
    fn mount_matches_routes(self) -> Self {
        let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL + BASE_MATCH_URL);
        self.mount(base_url, routes![routes::create_match,])
    }
}
