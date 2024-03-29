use rocket::{Build, Rocket};

use crate::presenter::routes::{
    routes_setup::BASE_API_URL,
    test::{routes, RoutesTestInitialized},
};

impl RoutesTestInitialized for Rocket<Build> {
    fn mount_test_routes(self) -> Self {
        self.mount(
            BASE_API_URL,
            routes![routes::hello, routes::hello_username, routes::error],
        )
    }
}
