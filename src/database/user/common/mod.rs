mod common_database;

pub trait UserCommonDatabase {
    fn get_user_count(&self) -> UserCommonOutcome<String>;
}

pub enum UserCommonOutcome<T> {
    Ok(T),
    Other,
}
