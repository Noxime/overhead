use ::discord::{
    E,
    Snowflake,
    Snowable,
    { // function imports
        deserialize_path 
    }
};
// use serde::de::{
//     self,
//     Deserialize,
//     Deserializer,
// };

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum Guild {
//     Partial {
//         #[serde(flatten)]
//         common: GuildCommon,
//     },
//     Full {
//         #[serde(flatten)]
//         common: GuildCommon,
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    owner: Option<bool>,
    permissions: u64,
    // TODO: rest of fields
}

// #[derive(Debug, Clone, Serialize)]
// struct FullGuild {

// }

// impl<'de> Deserialize<'de> for Guild {
//     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
//         //let mut map = JsonMap::deserialize(deserializer)?;
//         Ok(Self {
//             id: Snowflake::new(0),
//             name: "".into(),
//             icon: None,
//             owner: None,
//             permissions: 0,
//             full: None,
//         })
//     }
// }