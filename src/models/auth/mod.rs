pub mod google;

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: Vec<JwksKey>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwksKey {
    pub alg: String,
    pub kty: String,
    #[serde(rename="use")]
    pub useing: String,
    pub x5c: Vec<String>,
    pub n: String,
    pub e: String,
    pub kid: String,
    pub x5: String
}
