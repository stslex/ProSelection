use rocket::Rocket;

use crate::routes::{
    routes_setup::BASE_API_URL,
    test::{routes, RoutesTestInitialized},
};

impl RoutesTestInitialized for Rocket {
    fn mount_test_routes(self) -> Self {
        self.mount(
            BASE_API_URL,
            routes![routes::hello, routes::helloUsername, routes::error],
        )
    }
}
