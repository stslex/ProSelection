use rocket::Rocket;

use crate::routes::{
    auth::{routes, RoutesAuthInitialized},
    routes_setup::BASE_API_URL,
};

impl RoutesAuthInitialized for Rocket {
    fn mount_auth_routes(self) -> Self {
        self.mount(BASE_API_URL, routes![routes::login, routes::registration])
    }
}
