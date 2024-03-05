mod favourite;
mod follower;
mod matches;
mod routes;
mod routes_setup;

pub trait RoutesUserInitialized {
    fn mount_user_routes(self) -> Self;
}
