mod route_mount;
mod routes;

pub trait RouteMatchesInitialize {
    fn mount_matches_routes(self) -> Self;
}
