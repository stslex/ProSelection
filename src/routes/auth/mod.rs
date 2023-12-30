mod auth_objects;
mod routes;
mod routes_setup;
pub mod validators;

#[async_trait]
pub trait RoutesAuthInitialized {
    async fn mount_auth_routes(self) -> Self;
}
