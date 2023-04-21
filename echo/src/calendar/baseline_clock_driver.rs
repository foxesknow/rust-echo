use chrono::*;

use super::clock_driver::ClockDriver;

pub struct BaselineClockDriver
{
    duration: Duration
}

impl BaselineClockDriver
{
    pub fn from_utc(baseline: DateTime<Utc>) -> Self
    {
        let utc_now = Utc::now();
        let duration = utc_now - baseline;

        Self
        {
            duration: duration
        }
    }    

    pub fn from_local(baseline: DateTime<Local>) -> Self
    {
        let utc: DateTime<Utc> = DateTime::from(baseline);
        Self::from_utc(utc)
    }
}

impl ClockDriver for BaselineClockDriver
{
    fn utc_now(&self) -> DateTime<Utc>
    {
        Utc::now() - self.duration
    }
    
    fn now(&self) -> DateTime<Local>
    {
        let utc_now = self.utc_now();
        DateTime::from(utc_now)
    }
}