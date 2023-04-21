use chrono::*;

use super::clock_driver::ClockDriver;

pub struct SystemClockDriver
{
}

impl SystemClockDriver
{
    pub fn new() -> Self
    {
        SystemClockDriver{}
    }
}

impl ClockDriver for SystemClockDriver
{
    fn utc_now(&self) -> DateTime<Utc>
    {
        Utc::now()
    }
    
    fn now(&self) -> DateTime<Local>
    {
        Local::now()
    }
}