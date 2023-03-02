#[macro_use]
extern crate rocket;
extern crate oxide_auth;

pub(self) use crate::state::Oauth2State;

mod routes;
mod state;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/oauth2/", routes![routes::authorize, routes::token])
        .manage(Oauth2State::new())
}
