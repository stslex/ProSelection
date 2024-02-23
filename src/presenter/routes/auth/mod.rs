mod objects;
mod routes;
mod routes_setup;
pub mod validators;

pub trait RoutesAuthInitialized {
    fn mount_auth_routes(self) -> Self;
}
