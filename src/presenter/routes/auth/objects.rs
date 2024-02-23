use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LoginRequest<'a> {
    #[serde(rename = "login")]
    pub login: &'a str,
    #[serde(rename = "password")]
    pub password: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RefreshRequest<'a> {
    #[serde(rename = "refresh_token")]
    pub refresh_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RegistrationRequest<'a> {
    #[serde(rename = "login")]
    pub login: &'a str,
    #[serde(rename = "username")]
    pub username: &'a str,
    #[serde(rename = "password")]
    pub password: &'a str,
}
