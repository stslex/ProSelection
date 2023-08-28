use rocket::Rocket;

use crate::routes::routes_setup::BASE_API_URL;
use crate::routes::user::routes;
use crate::routes::user::RoutesUserInitialized;

impl RoutesUserInitialized for Rocket {
    fn mount_user_routes(self) -> Self {
        let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL);
        self.mount(base_url, routes![routes::get_user_count])
    }
}

const BASE_USER_URL: &str = "/user";