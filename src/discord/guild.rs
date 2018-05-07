use ::discord::{
    E,
    Snowflake,
    Snowable,
    { // function imports
        deserialize_path 
    }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    id: Snowflake,
    name: String,
    // TODO: Rest of fields
    full: Option<FullGuild>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FullGuild {
}
