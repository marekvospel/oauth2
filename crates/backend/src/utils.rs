use base64::Engine;
use rocket::{
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};

pub struct Authorization(pub Option<String>);

#[async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(Authorization(
            request
                .headers()
                .get_one("Authorization")
                .map(|v| v.to_string()),
        ))
    }
}

pub enum BasicAuth {
    NotPresent,

    Present(BasicAuthCredentials),
}

pub struct BasicAuthCredentials {
    pub username: String,

    pub password: String,
}

impl BasicAuth {
    pub fn get_username(&self) -> Option<String> {
        match self {
            BasicAuth::NotPresent => None,
            BasicAuth::Present(auth) => Some(auth.username.clone()),
        }
    }

    pub fn get_password(&self) -> Option<String> {
        match self {
            BasicAuth::NotPresent => None,
            BasicAuth::Present(auth) => Some(auth.password.clone()),
        }
    }
}

impl From<String> for BasicAuth {
    fn from(value: String) -> Self {
        if value.len() < 7 || &value[..6] != "Basic " {
            return BasicAuth::NotPresent;
        }

        let decoded = String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&value[6..])
                .unwrap(),
        )
        .unwrap();

        match decoded.split_once(":") {
            Some((username, password)) => BasicAuth::Present(BasicAuthCredentials {
                username: username.to_string(),
                password: password.to_string(),
            }),
            None => BasicAuth::NotPresent,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let headers: Vec<_> = request.headers().get("Authorization").collect();

        Outcome::Success(match headers.len() {
            1 => BasicAuth::from(headers[0].to_string()),
            _ => BasicAuth::NotPresent,
        })
    }
}
