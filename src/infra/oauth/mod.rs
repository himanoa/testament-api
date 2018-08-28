pub mod google;
use std::error::Error;

pub trait OAuthProvider {
    fn generate_authorize_url(&self, state: String) -> Result<String, Box<Error>>;
    fn get_token(&self, code: String) -> Result<String, Box<Error>>;
    fn verify_token(&self, token: String) -> Result<(), Box<Error>>;
}
