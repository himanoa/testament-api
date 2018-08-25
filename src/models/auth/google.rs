#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    id_token: String,
    expire_in: String,
    token_type: String,
    refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequestParams {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}
