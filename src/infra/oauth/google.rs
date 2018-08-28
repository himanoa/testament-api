use oauth2::prelude::*;
use url::Url;
use infra::oauth::OAuthProvider;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
             TokenUrl, TokenResponse, ExtraTokenFields, Client};
use oauth2::basic::{BasicTokenType, BasicErrorResponseType};
use serde_json;
use failure;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GoogleTokenField {
    pub id_token: String
}

impl ExtraTokenFields for GoogleTokenField {}
impl GoogleTokenField {
    pub fn id_token(&self) -> &str {
        self.id_token.as_str()
    }
}

pub type GoogleClient = Client<GoogleTokenField, BasicTokenType, BasicErrorResponseType>;
pub struct GoogleProvider {
    client: GoogleClient
}

pub type GoogleTokenResponse = TokenResponse<GoogleTokenField, BasicTokenType>;


impl GoogleProvider {
    fn new(client_id: String, client_secret: String, auth_url: String, token_url: String, redirect_url: String) -> GoogleProvider {
        let client = GoogleClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(Url::parse(&auth_url).expect("Invalid auth url")),
            Some(TokenUrl::new(Url::parse(&token_url).expect("Invalid token url")))
        ).set_redirect_url(
            RedirectUrl::new(
                Url::parse(&redirect_url)
                    .expect("Invalid redirect URL")
            )
        );
        GoogleProvider {
            client: client
        }
    }
}

impl OAuthProvider for GoogleProvider {
    fn generate_authorize_url(&self, state: &str) -> Result<(Url, String), failure::Error> {
        let (authorize_url, state) = self.client.authorize_url(|| CsrfToken::new(state.to_string()));
        Ok((authorize_url, state.secret().clone()))
    }

    fn get_token(&self, code: &str) -> Result<String, failure::Error> {
        let auth_code = AuthorizationCode::new(code.to_string());
        let res = self.client.exchange_code(auth_code)?;
        let google_token_response = GoogleTokenResponse::from_json(serde_json::to_string(&res)?.as_str());
        Ok(google_token_response?.extra_fields().id_token.clone())
    }
}
