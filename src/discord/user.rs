use ::discord::{
    E,
    Snowflake,
    Snowable,
    Guild,
    { // function imports
        deserialize_path 
    }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Snowflake,
    #[serde(rename = "username")]
    name: String,
    #[serde(rename = "discriminator")]
    disc: Option<String>,
    avatar: Option<String>,
    bot: Option<bool>,
    #[serde(rename = "mfa_enabled")]
    mfa: Option<bool>,
    verified: Option<bool>,
    email: Option<String>,
}

impl User {
    // get bot user
    pub fn me() -> Result<Self, E> {
        deserialize_path("/users/@me")
    }

    // get any user by ID
    pub fn get<T: Into<Snowflake>>(user: T) -> Result<Self, E> {
        deserialize_path(format!("/users/{}", user.into()).as_str())
    }

    pub fn guilds() -> Result<Vec<Guild>, E> {
        deserialize_path("/users/@me/guilds")
    }
}

impl Snowable for User {
    fn id(&self) -> Snowflake {
        self.id
    }
}