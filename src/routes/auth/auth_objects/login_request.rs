use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LoginRequest {
    #[serde(rename = "login")]
    pub login: &'static str,
    #[serde(rename = "password")]
    pub password: &'static str,
}
