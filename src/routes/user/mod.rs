mod routes;
mod routes_setup;

#[async_trait]
pub trait RoutesUserInitialized {
    async fn mount_user_routes(self) -> Self;
}
