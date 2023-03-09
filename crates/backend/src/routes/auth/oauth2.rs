use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};

fn discord_oauth() -> BasicClient {
    BasicClient::new(
        ClientId::new("1083417514463215676".to_string()),
        Some(ClientSecret::new("".to_string())),
        AuthUrl::new("https://discord.com/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://localhost:8000/auth/discord".to_string()).unwrap())
}

#[get("/auth/discord", rank = 1)]
pub async fn discord_redirect() -> Redirect {
    let client = discord_oauth();

    let (url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Redirect::to(url.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct DiscordUserResponse {
    id: String,
}

#[get("/auth/discord?<code>", rank = 0)]
pub async fn discord_authorize(code: String) {
    let client = discord_oauth();

    let token = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await;

    if token.is_err() {
        error!("{}", token.unwrap_err());
        return;
    }
    let token = token.unwrap();

    let response = reqwest::Client::new()
        .get("https://discord.com/api/users/@me")
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token().secret()),
        )
        .send()
        .await;

    if response.is_err() {
        error!("{}", response.unwrap_err());
        return;
    }
    let response = response.unwrap().json::<DiscordUserResponse>().await;
    if response.is_err() {
        error!("{}", response.unwrap_err());
        return;
    }
    let response = response.unwrap();

    println!("{response:?}")
}
