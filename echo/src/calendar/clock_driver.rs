use chrono::*;

pub trait ClockDriver
{
    fn utc_now(&self) -> DateTime<Utc>;
    fn now(&self) -> DateTime<Local>;
}