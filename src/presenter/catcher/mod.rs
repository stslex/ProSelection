mod catcher;

pub trait AppCatcher {
    fn mount_catcher(self) -> Self;
}
