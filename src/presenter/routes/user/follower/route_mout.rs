use rocket::{Build, Rocket};

use super::RouteFollowerInitialize;

impl RouteFollowerInitialize for Rocket<Build> {
    fn mount_follower_routes(self) -> Self {
        // let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL);
        self
    }
}
