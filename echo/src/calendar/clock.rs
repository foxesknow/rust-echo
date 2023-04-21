use chrono::*;
use std::sync::*;

use super::{clock_driver::ClockDriver, system_clock_driver::SystemClockDriver};

static INIT: Once = Once::new();
static mut INSTALLED_CLOCK: Option<Mutex<Option<Box<dyn ClockDriver>>>> = None;

fn clock_mutex<'a>() -> &'a Mutex<Option<Box<dyn ClockDriver>>>
{
    INIT.call_once(||
    {
        unsafe
        {
            INSTALLED_CLOCK = Some(Mutex::new(Some(Box::new(SystemClockDriver::new()))));
        }
    });

    unsafe
    {
        INSTALLED_CLOCK.as_ref().unwrap()
    }
}

pub fn utc_now() -> DateTime<Utc>
{
    let mutex = clock_mutex();
    let existing_clock = mutex.lock().unwrap();
    existing_clock.as_ref().unwrap().as_ref().utc_now()
}

pub fn now() -> DateTime<Local>
{
    let mutex = clock_mutex();
    let existing_clock = mutex.lock().unwrap();
    existing_clock.as_ref().unwrap().as_ref().now()
}

pub fn install(clock: Box<dyn ClockDriver>) -> Box<dyn ClockDriver>
{
    let mutex = clock_mutex();
    let mut existing_clock = mutex.lock().unwrap();
    existing_clock.replace(clock).unwrap()
}

pub fn quick_switch<T>(clock: Box<dyn ClockDriver>, f: impl FnOnce() -> T) -> T
{
    let old_clock = install(clock);
    let result = f();
    install(old_clock);

    result
}