use oauth2::basic::BasicClient;
use oauth2::{AccessToken, AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenType, TokenUrl};
use oauth2::url::Url;
use crate::*;

pub struct DiscordOAuth {
    client: BasicClient,
    reqwest_client: reqwest::Client,
    auth_url: Url,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct DiscordUser {
    pub id: u64,
    pub email: Option<String>,
}

macro_rules! discord_api_url {
    ($url: expr) => {
        format!("https://discord.com/api/v8/{}", $url)
    }
}

impl DiscordOAuth {
    pub fn new() -> DiscordOAuth {
        let client = BasicClient::new(
            ClientId::new(std::env::var("CLIENT_ID")
                .expect("CLIENT_ID must be set in environment")),
            Some(ClientSecret::new(std::env::var("CLIENT_SECRET")
                .expect("CLIENT_SECRET must be set in environment"))),
            AuthUrl::new(discord_api_url!("oauth2/authorize")).unwrap(),
            Some(TokenUrl::new(discord_api_url!("oauth2/token")).unwrap()),
        ).set_redirect_uri(RedirectUrl::new(
            std::env::var("REDIRECT_URI")
                .expect("REDIRECT_URI must be set in environment")
        ).unwrap());
        let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let auth_request = client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge)
            .add_scope(Scope::new("identify".into()))
            .add_scope(Scope::new("email".into()));
        let (auth_url, _csrf_token) = auth_request.url();
        Self {
            client,
            reqwest_client: reqwest::Client::new(),
            auth_url,
        }
    }

    pub fn get_auth_url(&self) -> &Url {
        &self.auth_url
    }

    pub async fn with_code(&self, code: String) -> Result<DiscordUser> {
        let token_result = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client).await
            .map_err(|err| Error::Any(err.into()))?;
        self.get_user(token_result.token_type(), token_result.access_token()).await
    }

    pub async fn get_user<TT>(&self, token_type: &TT, access_token: &AccessToken) -> Result<DiscordUser>
        where TT: TokenType {
        Ok(self.reqwest_client
            .get(discord_api_url!("users/@me"))
            .header("Authorization",
                    format!("{:?} {}", token_type, access_token.secret()))
            .send().await?
            .json().await?
        )
    }
}