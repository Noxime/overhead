use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Snowflake(
    #[serde(with = "string")]
    u64
);

impl Snowflake {
    // the creation time of this snowflake (and the object associated with it)
    pub fn time_millis(&self) -> u64 {
        (self.0 >> 22) + 1420070400000
    }
}


pub trait Snowable {
    fn id(&self) -> Snowflake;
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for Snowflake
where
    T: Snowable
{
    fn from(other: T) -> Self {
        other.id()
    }
}

impl<'a> Snowable for &'a str {
    fn id(&self) -> Snowflake {
        Snowflake(match self.parse() {
            Ok(v) => { 
                warn!("&str ({}) converted to snowflake! this is should only be used for testing, as it can fail", v);
                v
            },
            Err(why) => {
                error!("&str could not be converted to snowflake: {}", why);
                0
            }
        })
    }
}

mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where T: Display,
              S: Serializer
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where T: FromStr,
              T::Err: Display,
              D: Deserializer<'de>
    {
        String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
    }
}