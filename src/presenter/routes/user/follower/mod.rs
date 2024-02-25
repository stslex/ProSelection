mod route_mout;

pub trait RouteFollowerInitialize {
    fn mount_follower_routes(self) -> Self;
}
