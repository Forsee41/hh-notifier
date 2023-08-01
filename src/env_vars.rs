pub struct EnvVars {
    pub token: String,
    pub channel_id: u64,
    pub discord_role_name: String,
    pub start_hour: u64,
    pub end_hour: u64,
    pub time_shift: u64,
}

pub enum EnvVarsLoadingError {
    Token,
    ChannelId,
    ChannelIdParsing,
    DiscordRoleName,
    StartHour,
    StartHourParsing,
    EndHour,
    EndHourParsing,
    TimeShift,
    TimeShiftParsing,
}

impl EnvVars {
    /// Loads and parses environment variables.
    /// Returns a result with either a struct with corresponding fields ,or an enum variant
    /// of missing/unparseable env variable.
    /// Check .env-example for a list of required env vars.
    /// Consider loading dotenv in prior.
    pub fn load() -> Result<EnvVars, EnvVarsLoadingError> {
        let channel_id: u64;
        let start_hour: u64;
        let end_hour: u64;
        let time_shift: u64;

        let token: String = match std::env::var("DISCORD_BOT_TOKEN") {
            Err(_) => return Err(EnvVarsLoadingError::Token),
            Ok(val) => val,
        };
        let discord_role_name: String = match std::env::var("DISCORD_ROLE_NAME") {
            Err(_) => return Err(EnvVarsLoadingError::Token),
            Ok(val) => val,
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
                Ok(val) => time_shift = val,
            },
        };
        Ok(EnvVars {
            token,
            channel_id,
            discord_role_name,
            start_hour,
            end_hour,
            time_shift,
        })
    }
}
