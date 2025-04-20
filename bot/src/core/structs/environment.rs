use std::error::Error;

pub enum EnvType {
    DiscordToken,
}

impl EnvType {
    fn as_str(&self) -> &'static str {
        match self {
            EnvType::DiscordToken => "DISCORD_TOKEN",
        }
    }
}
pub struct Environment {
    pub key: EnvType,
    pub value: String,
}

impl Environment {
    pub fn new(key: EnvType) -> Result<Self, Box<dyn Error>> {
        let token = dotenv::var(key.as_str())?;

        Ok(Self { key, value: token })
    }
}
