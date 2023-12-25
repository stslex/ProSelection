mod auth;
mod route_objects;
mod routes_setup;
mod swagger;
mod test;
mod user;

#[async_trait]
pub trait RoutesInitialized {
    async fn mount_routes(self) -> Self;
}
