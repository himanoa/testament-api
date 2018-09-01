use base64;
use failure;
use infra::oauth::{Claims, OAuthProvider, VerifyTokenError};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use oauth2::basic::{BasicErrorResponseType, BasicTokenType};
use oauth2::prelude::*;
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, ExtraTokenFields,
    RedirectUrl, TokenResponse, TokenUrl,
};
use reqwest;
use serde_json;
use url::Url;

use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GoogleTokenField {
    pub id_token: String,
}

impl ExtraTokenFields for GoogleTokenField {}
impl GoogleTokenField {
    pub fn id_token(&self) -> &str {
        self.id_token.as_str()
    }
}

pub type GoogleClient = Client<GoogleTokenField, BasicTokenType, BasicErrorResponseType>;
pub struct GoogleProvider {
    client: GoogleClient,
}

pub type GoogleTokenResponse = TokenResponse<GoogleTokenField, BasicTokenType>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WellKnown {
    pub jwks_url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JwksKey {
    pub alg: String,
    pub kty: String,
    #[serde(rename = "use")]
    pub useing: String,
    pub x5c: Vec<String>,
    pub n: String,
    pub e: String,
    pub kid: String,
    pub x5: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Jwks {
    pub keys: Vec<JwksKey>,
}

#[derive(Debug, Deserialize)]
pub struct Envelope {
    pub alg: String,
    pub kid: String,
}

fn algolitm_from_str(s: &str) -> Result<Algorithm, VerifyTokenError> {
    match s {
        "HS256" => Ok(Algorithm::HS256),
        "HS384" => Ok(Algorithm::HS384),
        "HS512" => Ok(Algorithm::HS512),
        "RS256" => Ok(Algorithm::HS256),
        "RS384" => Ok(Algorithm::HS384),
        "RS512" => Ok(Algorithm::HS512),
        _ => Err(VerifyTokenError::UnknownAlgolithmError { msg: s.to_string() }),
    }
}

impl GoogleProvider {
    fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    ) -> GoogleProvider {
        let client = GoogleClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(Url::parse(&auth_url).expect("Invalid auth url")),
            Some(TokenUrl::new(
                Url::parse(&token_url).expect("Invalid token url"),
            )),
        ).set_redirect_url(RedirectUrl::new(
            Url::parse(&redirect_url).expect("Invalid redirect URL"),
        ));
        GoogleProvider { client: client }
    }
}

impl OAuthProvider for GoogleProvider {
    fn generate_authorize_url(&self, state: &str) -> Result<(Url, String), failure::Error> {
        let (authorize_url, state) = self
            .client
            .authorize_url(|| CsrfToken::new(state.to_string()));
        Ok((authorize_url, state.secret().clone()))
    }

    fn get_token(&self, code: &str) -> Result<String, failure::Error> {
        let auth_code = AuthorizationCode::new(code.to_string());
        let res = self.client.exchange_code(auth_code)?;
        let google_token_response =
            GoogleTokenResponse::from_json(serde_json::to_string(&res)?.as_str());
        Ok(google_token_response?.extra_fields().id_token.clone())
    }

    fn verify_token(&self, token: &str, well_knwon_url: &str) -> Result<Claims, VerifyTokenError> {
        let segments: Vec<&str> = token.split(".").collect();
        if segments.len() <= 2 {
            return Err(VerifyTokenError::SegmentSplitError {
                msg: "splited segments length <= 2".to_string(),
            });
        }
        let envelope: Envelope = match base64::decode_config(segments[0], base64::STANDARD) {
            Ok(json) => serde_json::from_str(String::from_utf8_lossy(&json).as_ref())?,
            Err(_) => {
                return Err(VerifyTokenError::EnvelopeDecodeError {
                    msg: "".to_string(),
                });
            }
        };
        let well_known: WellKnown = reqwest::get(well_knwon_url)?.json()?;
        let jwks: Jwks = reqwest::get(well_known.jwks_url.as_str())?.json()?;
        let key = jwks.keys.iter().find(|ref val| val.kid == envelope.kid)?;
        let algolithm: Algorithm = algolitm_from_str(key.alg.as_str())?;
        let validation: Validation = Validation {
            algorithms: Some(vec![algolithm]),
            ..Validation::default()
        };
        let token_data: TokenData<Claims> =
            match decode::<Claims>(token, key.n.as_bytes(), &validation) {
                Ok(c) => Ok(c),
                Err(e) => match *e.kind() {
                    ErrorKind::InvalidToken => Err(VerifyTokenError::InvalidTokenError),
                    ErrorKind::InvalidIssuer => Err(VerifyTokenError::InvalidIssuerError),
                    _ => panic!(),
                },
            }?;
        Ok(token_data.claims)
    }
}
