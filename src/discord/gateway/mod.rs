use websocket::{ 
    ClientBuilder,
    Message,
    client::sync::Client,
    message::{
        OwnedMessage
    },
    stream::sync::{
        TlsStream,
        TcpStream,
    }
};
use serde_json;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
enum Payload {
    #[serde(rename = "3", rename_as = "int")]
    Hello {
        heartbeat_interval: usize,
    },
    Unknown, 
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Hello {
    heartbeat_interval: usize
}


const WS_ADDR: &'static str = "wss://gateway.discord.gg/?v=6&encoding=json";

lazy_static! {
    static ref CLIENT: Mutex<Client<TlsStream<TcpStream>>> = {
        Mutex::new(ClientBuilder::new(WS_ADDR)
            .unwrap()
            .connect_secure(None)
            .unwrap())
    };
}

pub fn poll() {
    if let Ok(OwnedMessage::Text(response)) = CLIENT.lock().unwrap().recv_message() {
        match serde_json::from_str::<Payload>(&response) {
            Ok(value) => { 
                debug!("Gateway: {:#?}", value);
            },
            Err(why) => {
                warn!("Gateway deserialize failed");
                warn!("{:#?}", why);
            }
        }
    } else {
        warn!("Gateway receive failed");
    }
}