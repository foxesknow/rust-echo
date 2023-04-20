use std::collections::*;

use crate::settings::*;

pub struct QuickSettings
{
    values : HashMap<String, String>
}

impl QuickSettings 
{
    pub fn new(values : &[(&str, &str)]) -> Self
    {
        let mut settings = QuickSettings
        {
            values : HashMap::new()
        };

        for pair in values.iter()
        {
            let key = pair.0.to_lowercase();
            let value = pair.1.to_string();

            settings.values.insert(key, value);
        }

        settings
    }   
}

impl Settings for QuickSettings
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>
    {
        let key = name.to_lowercase();
        let value = self.values.get(&key);

        match value
        {
            Some(&ref x) => Ok(x.to_string()),
            _            => Err(SettingError::NotFound)
        }        
    }
}