use rocket::{Build, Rocket};

use super::RouteMatchesInitialize;

impl RouteMatchesInitialize for Rocket<Build> {
    fn mount_matches_routes(self) -> Self {
        // let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL);
        self
    }
}
