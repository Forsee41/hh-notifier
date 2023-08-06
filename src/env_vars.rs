use crate::handler::BotConfig;

pub struct EnvVars {
    pub token: String,
    pub channel_id: u64,
    pub role_id: u64,
    pub start_hour: u64,
    pub end_hour: u64,
    pub notify_shift: u64,
}

#[derive(Debug)]
pub enum EnvVarsLoadingError {
    Token,
    ChannelId,
    ChannelIdParsing,
    RoleId,
    RoleIdParsing,
    StartHour,
    StartHourParsing,
    EndHour,
    EndHourParsing,
    TimeShift,
    TimeShiftParsing,
}

impl EnvVars {
    pub fn to_bot_config(&self) -> BotConfig {
        BotConfig {
            channel_id: self.channel_id,
            role_id: self.role_id,
            start: self.start_hour,
            end: self.end_hour,
            notify_shift: self.notify_shift,
        }
    }
    /// Loads and parses environment variables.
    /// Returns a result with either a struct with corresponding fields ,or an enum variant
    /// of missing/unparseable env variable.
    /// Check .env-example for a list of required env vars.
    /// Consider loading dotenv in prior.
    pub fn load() -> Result<EnvVars, EnvVarsLoadingError> {
        let channel_id: u64;
        let start_hour: u64;
        let end_hour: u64;
        let notify_shift: u64;
        let role_id: u64;

        let token: String = match std::env::var("DISCORD_BOT_TOKEN") {
            Err(_) => return Err(EnvVarsLoadingError::Token),
            Ok(val) => val,
        };
        match std::env::var("DISCORD_ROLE_ID") {
            Err(_) => return Err(EnvVarsLoadingError::RoleId),
            Ok(val) => match val.parse() {
                Err(_) => return Err(EnvVarsLoadingError::RoleIdParsing),
                Ok(val) => role_id = val,
            },
        };
        match std::env::var("DISCORD_CHANNEL_ID") {
            Err(_) => return Err(EnvVarsLoadingError::ChannelId),
            Ok(val) => match val.parse() {
                Err(_) => return Err(EnvVarsLoadingError::ChannelIdParsing),
                Ok(val) => channel_id = val,
            },
        };
        match std::env::var("HH_START_HOUR_UTC") {
            Err(_) => return Err(EnvVarsLoadingError::StartHour),
            Ok(val) => match val.parse() {
                Err(_) => return Err(EnvVarsLoadingError::StartHourParsing),
                Ok(val) => start_hour = val,
            },
        };
        match std::env::var("HH_FINISH_HOUR_UTC") {
            Err(_) => return Err(EnvVarsLoadingError::EndHour),
            Ok(val) => match val.parse() {
                Err(_) => return Err(EnvVarsLoadingError::EndHourParsing),
                Ok(val) => end_hour = val,
            },
        };
        match std::env::var("NOTIFICATION_TIME_SHIFT_MINUTES") {
            Err(_) => return Err(EnvVarsLoadingError::TimeShift),
            Ok(val) => match val.parse() {
                Err(_) => return Err(EnvVarsLoadingError::TimeShiftParsing),
                Ok(val) => notify_shift = val,
            },
        };
        Ok(EnvVars {
            token,
            channel_id,
            role_id,
            start_hour,
            end_hour,
            notify_shift,
        })
    }
}
