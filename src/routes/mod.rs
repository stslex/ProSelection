mod auth;
mod route_objects;
mod routes_setup;
mod test;
mod user;
mod swagger;

pub trait RoutesInitialized {
    fn mount_routes(self) -> Self;
}
