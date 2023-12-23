use rocket::{Build, Rocket};

use crate::routes::auth::RoutesAuthInitialized;
use crate::routes::swagger::SwaggerRouteInitialized;
use crate::routes::test::RoutesTestInitialized;
use crate::routes::user::RoutesUserInitialized;
use crate::routes::RoutesInitialized;

#[async_trait]
impl RoutesInitialized for Rocket<Build> {
    async fn mount_routes(self) -> Self {
        self.mount_test_routes()
            .mount_auth_routes()
            .mount_user_routes()
            .mount_swagger_route()
    }
}

pub const BASE_API_URL: &str = "/api/v1";
