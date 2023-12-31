use rocket::Rocket;

use crate::routes::auth::RoutesAuthInitialized;
use crate::routes::swagger::SwaggerRouteInitialized;
use crate::routes::test::RoutesTestInitialized;
use crate::routes::user::RoutesUserInitialized;
use crate::routes::RoutesInitialized;

impl RoutesInitialized for Rocket {
    fn mount_routes(self) -> Self {
        self.mount_test_routes()
            .mount_auth_routes()
            .mount_user_routes()
            .mount_swagger_route()
    }
}

pub const BASE_API_URL: &str = "/api/v1";
