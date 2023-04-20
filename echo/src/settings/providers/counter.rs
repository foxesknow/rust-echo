use crate::settings::Settings;

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
    fn get_setting(&mut self, name : &str) -> Option<String>
    {
        match name.to_lowercase().as_str()
        {
            "next" => Some(self.counter.fetch_add(1, Ordering::SeqCst).to_string()),
            _ => None
        }
    }
}

