#[macro_use]
extern crate rocket;
extern crate oxide_auth;

pub(self) use crate::routes::*;
pub(self) use crate::state::Oauth2State;

mod routes;
mod state;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/oauth2/", routes![oauth2::authorize, oauth2::token])
        .mount("/", routes![index])
        .manage(Oauth2State::new())
}
