pub mod google;
use failure;
use jsonwebtoken::errors;
use models::auth::Jwks;
use reqwest;
use serde_json;
use std;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub at_hash: String,
    pub exp: u64,
    pub iss: String,
    pub iat: u64,
}

#[derive(Debug, Fail)]
pub enum VerifyTokenError {
    #[fail(display = "Segments length error: {}", msg)]
    SegmentSplitError { msg: String },
    #[fail(display = "Decode error: {}", msg)]
    EnvelopeDecodeError { msg: String },
    #[fail(display = "Algorithm parse error: {}", msg)]
    UnknownAlgolithmError { msg: String },
    #[fail(display = "InvalidTokenError")]
    InvalidTokenError,
    #[fail(display = "InvalidIssuerError")]
    InvalidIssuerError,
    #[fail(display = "IO error")]
    Io(std::io::Error),
    #[fail(display = "Reqwest error")]
    Reqwest(reqwest::Error),
    #[fail(display = "JSON decode error")]
    SerdeJson(serde_json::Error),
    #[fail(display = "None error")]
    NoneError(std::option::NoneError),
}

impl From<std::io::Error> for VerifyTokenError {
    fn from(err: std::io::Error) -> VerifyTokenError {
        VerifyTokenError::Io(err)
    }
}

impl From<serde_json::Error> for VerifyTokenError {
    fn from(err: serde_json::Error) -> VerifyTokenError {
        VerifyTokenError::SerdeJson(err)
    }
}
impl From<reqwest::Error> for VerifyTokenError {
    fn from(err: reqwest::Error) -> VerifyTokenError {
        VerifyTokenError::Reqwest(err)
    }
}

impl From<std::option::NoneError> for VerifyTokenError {
    fn from(err: std::option::NoneError) -> VerifyTokenError {
        VerifyTokenError::NoneError(err)
    }
}

pub trait OAuthProvider {
    fn generate_authorize_url(&self, state: &str) -> Result<(String, String), failure::Error>;
    fn get_token(&self, code: &str) -> Result<String, failure::Error>;
    fn verify_token(&self, token: &str, well_knwon_url: &str) -> Result<Claims, VerifyTokenError>;
}
