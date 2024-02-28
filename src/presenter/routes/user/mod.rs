mod favourite;
mod follower;
mod matches;
mod objects;
mod routes;
mod routes_setup;

pub trait RoutesUserInitialized {
    fn mount_user_routes(self) -> Self;
}
