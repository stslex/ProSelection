mod route_mount;

pub trait RouteMatchesInitialize {
    fn mount_matches_routes(self) -> Self;
}
