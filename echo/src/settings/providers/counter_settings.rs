use crate::settings::*;

use std::sync::atomic::*;

pub struct CounterSettings
{
    counter : AtomicU64
}

impl CounterSettings
{
    pub fn new() -> Self
    {
        CounterSettings
        {
            counter : AtomicU64::new(1)
        }
    }
}

impl Settings for CounterSettings
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>
    {
        match name.to_lowercase().as_str()
        {
            "next" => Ok(self.counter.fetch_add(1, Ordering::SeqCst).to_string()),
            _      => Err(SettingError::NotFound)
        }
    }
}

