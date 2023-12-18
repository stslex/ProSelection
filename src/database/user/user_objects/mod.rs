pub mod user;

pub enum UserCommonOutcome<T> {
    Ok(T),
    Error,
}
