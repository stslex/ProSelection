mod auth_objects;
mod routes;
mod routes_setup;

pub trait RoutesAuthInitialized {
    fn mount_auth_routes(self) -> Self;
}
