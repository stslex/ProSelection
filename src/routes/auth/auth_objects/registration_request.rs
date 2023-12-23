use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RegistrationRequest {
    #[serde(rename = "login")]
    pub login: &'static str,
    #[serde(rename = "username")]
    pub username: &'static str,
    #[serde(rename = "password")]
    pub password: &'static str,
}
