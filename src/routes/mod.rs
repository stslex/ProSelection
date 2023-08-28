mod auth;
mod route_objects;
mod routes_setup;
mod test;

pub trait RoutesInitialized {
    fn mount_routes(self) -> Self;
}
