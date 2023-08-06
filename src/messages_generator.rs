use crate::handler::BotConfig;
use chrono::{prelude::*, Duration};
use serenity::model::{mention::Mention, prelude::RoleId};
use std::sync::Arc;

pub struct MessageManager {
    config: Arc<BotConfig>,
}
impl MessageManager {
    fn _next_hh_start_datetime(&self) -> DateTime<Utc> {
        let now = Utc::now();
        let hh_start = now
            .clone()
            .date_naive()
            .and_hms_opt(self.config.start as u32, 0, 0)
            .unwrap()
            .and_utc();
        if hh_start <= now {
            hh_start + Duration::days(1)
        } else {
            hh_start
        }
    }
    fn _next_hh_end_datetime(&self) -> DateTime<Utc> {
        let now = Utc::now();
        let hh_end = now
            .clone()
            .date_naive()
            .and_hms_opt(self.config.end as u32, 0, 0)
            .unwrap()
            .and_utc();
        if hh_end <= now {
            hh_end + Duration::days(1)
        } else {
            hh_end
        }
    }
    fn _duration_until_next_hh_start(&self) -> Duration {
        self._next_hh_start_datetime() - Utc::now()
    }
    fn _duration_until_next_hh_end(&self) -> Duration {
        self._next_hh_end_datetime() - Utc::now()
    }
    pub fn during_event_str(&self) -> String {
        let end_datetime = self._next_hh_end_datetime();
        format!(
            "Happy Hour ends <t:{}:R> at <t:{}:t>",
            end_datetime.timestamp(),
            end_datetime.timestamp(),
        )
    }
    pub fn before_event_str(&self) -> String {
        let start_datetime = self._next_hh_start_datetime();
        format!(
            "Happy Hour starts <t:{}:R> at <t:{}:t>",
            start_datetime.timestamp(),
            start_datetime.timestamp(),
        )
    }
    pub fn notify_str(&self) -> String {
        let role: RoleId = self.config.role_id.into();
        let mention = Mention::from(role);
        format!("{} Happy Hour!", mention)
    }
    pub fn info_str(&self) -> String {
        let next_end_time = self._next_hh_end_datetime();
        let next_start_time = self._next_hh_start_datetime();
        if next_start_time > next_end_time {
            self.during_event_str()
        } else {
            self.before_event_str()
        }
    }
    pub fn new(config: Arc<BotConfig>) -> Self {
        MessageManager { config }
    }
}
