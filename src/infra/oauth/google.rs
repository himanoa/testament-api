use oauth2::prelude::*;
use url::Url;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
             TokenUrl};
use oauth2::basic::BasicClient;

struct GoogleProvider {
    client: BasicClient
}

impl GoogleProvider {
    fn new(client_id: String, client_secret: String, auth_url: String, token_url: String, redirect_url: String) -> GoogleProvider {
        let basic_client = BasicClient::new(
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
            client: basic_client
        }
    }
}
