use crate::state::Oauth2State;
use oxide_auth_rocket::{OAuthFailure, OAuthRequest};
use rocket::response::Responder;
use rocket::State;

pub(crate) mod oauth2;

#[get("/")]
pub(crate) async fn index<'r>(
    oauth: OAuthRequest<'r>,
    state: &State<Oauth2State>,
) -> impl Responder<'r, 'r> {
    let protect = state
        .endpoint()
        .with_scopes(vec!["default-scope".parse().unwrap()])
        .resource_flow()
        .execute(oauth);

    match protect {
        Ok(grant) => Ok(grant.owner_id),
        Err(e) => match e {
            Ok(o) => Err(Ok(o)),
            Err(e) => Err(Err(e.pack::<OAuthFailure>())),
        },
    }
}
