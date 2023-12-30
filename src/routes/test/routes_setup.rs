use rocket::{Build, Rocket};

use crate::routes::{
    routes_setup::BASE_API_URL,
    test::{routes, RoutesTestInitialized},
};

#[async_trait]
impl RoutesTestInitialized for Rocket<Build> {
    async fn mount_test_routes(self) -> Self {
        self.mount(
            BASE_API_URL,
            routes![routes::hello, routes::hello_username, routes::error],
        )
    }
}
