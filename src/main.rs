extern crate reqwest;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod discord;
use discord::User;

fn main() {
    pretty_env_logger::init();
    debug!("{:#?}", User::guilds());
}
