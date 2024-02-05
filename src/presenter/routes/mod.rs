mod auth;
mod routes_setup;
mod swagger;
mod test;
mod user;

pub trait RoutesInitialized {
    fn mount_routes(self) -> Self;
}
