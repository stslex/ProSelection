use rocket::Rocket;

use crate::routes::{
    auth::{routes, RoutesAuthInitialized},
    routes_setup::BASE_API_URL,
};

impl RoutesAuthInitialized for Rocket {
    fn mount_auth_routes(self) -> Self {
        let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL);
        self.mount(base_url, routes![routes::login, routes::registration])
    }
}

const BASE_USER_URL: &str = "/passport";
