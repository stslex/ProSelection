mod routes;
mod routes_setup;

pub trait RoutesTestInitialized {
    fn mount_test_routes(self) -> Self;
}
