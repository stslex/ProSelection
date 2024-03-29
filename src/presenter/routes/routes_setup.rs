use rocket::{Build, Rocket};

use crate::presenter::routes::auth::RoutesAuthInitialized;
use crate::presenter::routes::test::RoutesTestInitialized;
use crate::presenter::routes::user::RoutesUserInitialized;
use crate::presenter::routes::RoutesInitialized;

use super::swagger::SwaggerRouteInitialized;

impl RoutesInitialized for Rocket<Build> {
    fn mount_routes(self) -> Self {
        self.mount_test_routes()
            .mount_auth_routes()
            .mount_user_routes()
            .mount_swagger_route()
    }
}

pub const BASE_API_URL: &str = "/api/v1";
