mod route_mount;
mod routes;

pub trait RouteFollowerInitialize {
    fn mount_follower_routes(self) -> Self;
}
