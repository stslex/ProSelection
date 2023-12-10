use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RefreshRequest<'a> {
    #[serde(rename = "refresh_token")]
    pub refresh_token: &'a str,
}
