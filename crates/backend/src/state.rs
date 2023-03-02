use oxide_auth::frontends::simple::endpoint::{Generic, Vacant};
use oxide_auth::primitives::prelude::*;
use oxide_auth::primitives::registrar::RegisteredUrl;
use std::sync::Mutex;

pub struct Oauth2State {
    authorizer: Mutex<AuthMap<RandomGenerator>>,
    issuer: Mutex<TokenMap<RandomGenerator>>,
    registrar: Mutex<ClientMap>,
}

impl Oauth2State {
    pub fn new() -> Self {
        Oauth2State {
            authorizer: Mutex::new(AuthMap::new(RandomGenerator::new(16))),
            issuer: Mutex::new(TokenMap::new(RandomGenerator::new(16))),
            registrar: Mutex::new(
                vec![Client::public(
                    "hacker",
                    RegisteredUrl::Semantic("http://localhost:8000/redirect".parse().unwrap()),
                    "default-scope".parse().unwrap(),
                )]
                .into_iter()
                .collect(),
            ),
        }
    }

    pub fn endpoint(&self) -> Generic<impl Registrar + '_, impl Authorizer + '_, impl Issuer + '_> {
        Generic {
            authorizer: self.authorizer.lock().unwrap(),
            registrar: self.registrar.lock().unwrap(),
            issuer: self.issuer.lock().unwrap(),
            solicitor: Vacant,
            response: Vacant,
            scopes: Vacant,
        }
    }
}
