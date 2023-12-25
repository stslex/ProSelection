use rocket::{Build, Rocket};

use crate::routes::auth::RoutesAuthInitialized;
use crate::routes::test::RoutesTestInitialized;
use crate::routes::user::RoutesUserInitialized;
use crate::routes::RoutesInitialized;

use super::swagger::SwaggerRouteInitialized;

#[async_trait]
impl RoutesInitialized for Rocket<Build> {
    async fn mount_routes(self) -> Self {
        self.mount_test_routes()
            .await
            .mount_auth_routes()
            .await
            .mount_user_routes()
            .await
            .mount_swagger_route()
            .await
    }
}

pub const BASE_API_URL: &str = "/api/v1";
