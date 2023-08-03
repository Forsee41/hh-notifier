use crate::handler::BotConfig;
use chrono::{prelude::*, Duration};
use std::sync::Arc;
use serenity::model::{mention::Mention, prelude::RoleId};

pub struct MessageManager {
    config: Arc<BotConfig>,
}
impl MessageManager {
    fn _now_tz(&self) -> DateTime<FixedOffset> {
        let tz = self._tz();
        let utc_now = Utc::now();
        utc_now.with_timezone(&tz)
    }
    fn _tz(&self) -> FixedOffset {
        let hour = 3600;
        let tz_shift_seconds: i32 = hour * self.config.tz_shift as i32;
        FixedOffset::east_opt(tz_shift_seconds).unwrap()
    }
    fn _next_hh_start_datetime(&self) -> DateTime<FixedOffset> {
        let now_tz = self._now_tz();
        let now = Utc::now();
        let hh_start = now_tz
            .clone()
            .date_naive()
            .and_hms_opt(self.config.start as u32, 0, 0)
            .unwrap();
        let hh_start_with_tz: DateTime<FixedOffset> =
            DateTime::<FixedOffset>::from_utc(hh_start, self._tz());
        if now.hour() as u64 >= self.config.start {
            hh_start_with_tz + Duration::days(1)
        } else {
            hh_start_with_tz
        }
    }
    fn _next_hh_end_datetime(&self) -> DateTime<FixedOffset> {
        let now_tz = self._now_tz();
        let now = Utc::now();
        let hh_end = now_tz
            .clone()
            .date_naive()
            .and_hms_opt(self.config.end as u32, 0, 0)
            .unwrap();
        let hh_end_with_tz: DateTime<FixedOffset> =
            DateTime::<FixedOffset>::from_utc(hh_end, self._tz());
        if now.hour() as u64 >= self.config.end {
            hh_end_with_tz + Duration::days(1)
        } else {
            hh_end_with_tz
        }
    }
    fn _duration_until_next_hh_start(&self) -> Duration {
        self._next_hh_start_datetime() - Utc::now().with_timezone(&self._tz())
    }
    fn _duration_until_next_hh_end(&self) -> Duration {
        self._next_hh_end_datetime() - Utc::now().with_timezone(&self._tz())
    }
    pub fn during_event_str(&self) -> String {
        let duration = self._duration_until_next_hh_end();
        let end_datetime = self._next_hh_end_datetime();
        format!(
            "Happy Hour ends in `{}h {}m` at `{}:{:0>2}`",
            duration.num_hours(),
            (duration.num_minutes() % 60) + 1,
            end_datetime.hour(),
            end_datetime.minute(),
        )
    }
    pub fn before_event_str(&self) -> String {
        let duration = self._duration_until_next_hh_start();
        let start_datetime = self._next_hh_start_datetime();
        format!(
            "Happy Hour starts in `{}h {}m` at `{}:{:0>2}`",
            duration.num_hours(),
            (duration.num_minutes() % 60) + 1,
            start_datetime.hour(),
            start_datetime.minute(),
        )
    }
    pub fn notify_str(&self) -> String {
        let role: RoleId = self.config.role_id.into();
        let mention = Mention::from(role);
        format!("{} Happy Hour!", mention)
    }
    pub fn info_str(&self) -> String {
        let next_end_time = self._next_hh_end_datetime();
        let next_start_time= self._next_hh_start_datetime();
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
