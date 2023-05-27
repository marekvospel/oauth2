use redis::{Client, Connection};
use rocket::{
    async_trait,
    fairing::Fairing,
    serde::{Deserialize, Serialize},
    Rocket,
};
use sea_orm::ConnectOptions;
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::time::Duration;

#[derive(Database, Debug)]
#[database("sea_orm")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Connection = sea_orm::DatabaseConnection;
    type Error = sea_orm::DbErr;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let mut options: ConnectOptions = config.url.into();
        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .sqlx_logging(config.sqlx_logging);
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RedisConfig {
    pub url: String,
}

pub struct RedisFairing;

#[rocket::async_trait]
impl Fairing for RedisFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Redis",
            kind: rocket::fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(
        &self,
        rocket: Rocket<rocket::Build>,
    ) -> rocket::fairing::Result<Rocket<rocket::Build>, Rocket<rocket::Build>> {
        let config = rocket
            .figment()
            .extract_inner::<RedisConfig>("databases.redis")
            .unwrap();

        let client = Client::open(config.url).unwrap();

        Ok(rocket.manage(client))
    }
}
