mod routes_setup;
mod routes;

pub trait RoutesAuthInitialized {
    fn mount_auth_routes(self) -> Self;
}