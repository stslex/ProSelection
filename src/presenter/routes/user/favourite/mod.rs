mod route_mount;
mod routes;

pub trait RouteFavouriteInitialize {
    fn mount_favourite_routes(self) -> Self;
}
