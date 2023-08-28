use rocket::Rocket;

use crate::routes::auth::RoutesAuthInitialized;
use crate::routes::RoutesInitialized;
use crate::routes::test::RoutesTestInitialized;
use crate::routes::user::RoutesUserInitialized;

impl RoutesInitialized for Rocket {
    fn mount_routes(self) -> Self {
        self
            .mount_test_routes()
            .mount_auth_routes()
            .mount_user_routes()
    }
}

pub const BASE_API_URL: &str = "/api-v1";
