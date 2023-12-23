use rocket::Build;
use rocket::Rocket;

use crate::routes::routes_setup::BASE_API_URL;
use crate::routes::user::routes;
use crate::routes::user::RoutesUserInitialized;

#[async_trait]
impl RoutesUserInitialized for Rocket<Build> {
    async fn mount_user_routes(self) -> Self {
        let base_url = &*(BASE_API_URL.to_owned() + BASE_USER_URL);
        self.mount(
            base_url,
            routes![
                routes::get_user_count,
                routes::get_current_user,
                routes::get_user,
                routes::get_user_by_username,
                routes::post_add_favourite,
                routes::delete_remove_favourite,
                routes::get_is_favourite,
                routes::post_follow,
                routes::delete_follow,
                routes::get_is_following,
            ],
        )
    }
}

const BASE_USER_URL: &str = "/user";
