mod routes;
mod routes_setup;

#[async_trait]
pub trait RoutesTestInitialized {
    async fn mount_test_routes(self) -> Self;
}
