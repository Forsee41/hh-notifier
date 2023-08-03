pub struct CronTime {
    start: u64,
    end_: u64,
    shift: u64,
}

#[derive(Debug)]
pub enum CronTimeCreationError {
    Start,
    End,
    Shift,
}

impl CronTime {
    pub fn new(start: u64, end: u64, shift: u64) -> Result<Self, CronTimeCreationError> {
        if start > 24 {
            return Err(CronTimeCreationError::Start);
        };
        if end > 24 {
            return Err(CronTimeCreationError::End);
        };
        if shift > 59 {
            return Err(CronTimeCreationError::Shift);
        };
        Ok(CronTime {
            start,
            shift,
            end_: end,
        })
    }
    pub fn notify(&self) -> String {
        let notify_minutes = 60 - self.shift;
        let notify_hour = if self.shift > 0 {
            self.start - 1
        } else {
            self.start
        };
        format!("40 {} {} * * *", notify_minutes, notify_hour)
    }
    pub fn update(&self) -> String {
        "0 * * * * *".to_owned()
    }
    pub fn end(&self) -> String {
        format!("20 0 {} * * *", self.end_)
    }
}
