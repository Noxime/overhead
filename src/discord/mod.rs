use reqwest::{
    Client,
    Response,
    header::{
        Authorization,
        UserAgent,
        Headers,
    },
};
use serde::{ Deserialize };

mod snowflake;
mod user;
mod guild;

pub use ::discord::snowflake::{ Snowflake, Snowable };
pub use ::discord::user::User;
pub use ::discord::guild::{ Guild };

const BASE: &'static str = "https://discordapp.com/api/v6/";
lazy_static! {
    static ref CLIENT: Client = {
        let mut headers = Headers::new();
        headers.set(Authorization("Bot NDQxNzEzMjQyODgxMTMwNDk2.DdIUPw.8Ld8HMEOH3W-qIBhFFuxIfqfVPs".to_string()));
        headers.set(UserAgent::new(format!("DiscordBot (https://noxim.xyz/bot, {}) ALPHA/POC", env!("CARGO_PKG_VERSION"))));

        Client::builder()
        .default_headers(headers)
        .build().expect("Failed to build Reqwest client")
    };
}

#[derive(Debug, Clone)]
pub enum E {
    Reqwest(String),
    Deserialize,
}

fn basic_request<'a>(path: &'a str) -> Result<Response, E> {
    CLIENT.get(format!("{}/{}", BASE, path).as_str())
    .send()
    .map_err(|e| E::Reqwest(format!("{:?}", e)))
}

fn deserialize_path<'a, T>(path: &'a str) -> Result<T, E>
where
    for<'de> T: Deserialize<'de>,
{
    let mut response = basic_request(path)?;

    match response.json::<T>() {
        Ok(v) => Ok(v),
        Err(why) => {
            error!("deserialize failed: {}", why);
            Err(E::Deserialize)
        }
    }
}