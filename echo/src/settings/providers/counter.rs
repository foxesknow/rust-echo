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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_1() {
        let mut counter = CounterSettings::new();
        let value1 = counter.get_setting("next");
        assert_eq!(value1.unwrap(), "1".to_string());
    }

    #[test]
    fn next_returns_different_value() {
        let mut counter = CounterSettings::new();
        let value1 = counter.get_setting("next").unwrap();
        assert_eq!(value1, "1".to_string());

        let value2 = counter.get_setting("next").unwrap();
        assert_ne!(value1, value2);
    }
}

