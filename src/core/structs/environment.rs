use std::error::Error;

pub enum EnvType {
    DiscordToken,
    EvaluationChannel
}

impl EnvType {
    fn as_str(&self) -> &'static str {
        match self {
            EnvType::DiscordToken => "DISCORD_TOKEN",
            EnvType::EvaluationChannel => "EVALUATION_CHANNEL"
        }
    }
}

pub struct Environment {
    pub key: EnvType,
    pub value: String,
}

impl Environment {
    pub fn new(key: EnvType) -> Result<Environment, dotenv::Error> {
        let token = dotenv::var(key.as_str())?;

        Ok(Self { key, value: token })
    }
}
