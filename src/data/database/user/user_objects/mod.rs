pub mod user;

/// Represents the outcome of common user operations.
pub enum UserCommonOutcome<T> {
    Ok(T),
    Error,
}
