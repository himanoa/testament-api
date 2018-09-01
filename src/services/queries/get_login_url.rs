use infra::oauth::google::{GoogleProvider, GoogleProviderConfig};
use infra::oauth::OAuthProvider;

#[derive(Debug, Fail)]
pub enum GetLoginUrlError {
    #[fail(display = "Fail generate authorize_url")]
    FailedGenerateAuthorizeUrl,
}

pub fn generate_authorize_url(
    config: GoogleProviderConfig,
    state: &str
) -> Result<(String, String), GetLoginUrlError>{
    let provider : GoogleProvider = GoogleProvider::new(config);
    provider.generate_authorize_url(state).map_err(|_| {
        GetLoginUrlError::FailedGenerateAuthorizeUrl
    })
}
