pub mod google;

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwks {
    keys: Vec<JwksKey>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwksKey {
    alg: String,
    kty: String,
    #[serde(rename="use")]
    useing: String,
    x5c: Vec<String>,
    n: String,
    e: String,
    kid: String,
    x5: String
}
