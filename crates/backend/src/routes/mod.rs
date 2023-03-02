use crate::state::Oauth2State;
use oxide_auth::endpoint::{OwnerConsent, Solicitation};
use oxide_auth::frontends::simple::endpoint::FnSolicitor;
use oxide_auth_rocket::{OAuthFailure, OAuthRequest, OAuthResponse};
use rocket::{Data, State};

#[post("/authorize")]
pub(crate) async fn authorize<'r>(
    oauth: OAuthRequest<'r>,
    state: &State<Oauth2State>,
) -> Result<OAuthResponse<'r>, OAuthFailure> {
    state
        .endpoint()
        .with_solicitor(FnSolicitor(move |_: &mut _, _: Solicitation<'_>| {
            // TODO: check whether user is logged in and scopes are valid
            OwnerConsent::Authorized("marekvospel".into()) as OwnerConsent<OAuthResponse<'r>>
        }))
        .authorization_flow()
        .execute(oauth)
        .map_err(|err| err.pack::<OAuthFailure>())
}

#[post("/token", data = "<body>")]
pub(crate) async fn token<'r>(
    mut oauth: OAuthRequest<'r>,
    body: Data<'r>,
    state: &State<Oauth2State>,
) -> Result<OAuthResponse<'r>, OAuthFailure> {
    oauth.add_body(body).await;
    state
        .endpoint()
        .access_token_flow()
        .execute(oauth)
        .map_err(|err| err.pack::<OAuthFailure>())
}
