pub mod google;
use url::Url;
use failure;
use models::auth::Jwks

pub trait OAuthProvider {
    fn generate_authorize_url(&self, state: &str) -> Result<(Url, String), failure::Error>;
    fn get_token(&self, code: &str) -> Result<String, failure::Error>;
    // fn verify_token(&self, token: &str, well_knwon_url: &str) -> Result<(), Box<Error>>;
}
