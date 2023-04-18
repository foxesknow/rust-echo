use std::collections::*;

use crate::settings::Settings;

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
    fn get_setting(&mut self, name : &str) -> Option<String>
    {
        let key = name.to_lowercase();
        let value = self.values.get(&key);

        match value
        {
            Some(&ref x) => Some(x.to_string()),
            _            => None
        }        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() 
    {
        let values = [("name", "Jack")];
        let mut quick = QuickSettings::new(&values);

        let value = quick.get_setting("name");
        assert_eq!(value.unwrap(), "Jack".to_string());
    }

    #[test]
    fn no_such_value() 
    {
        let values = [("name", "Jack")];
        let mut quick = QuickSettings::new(&values);

        let value = quick.get_setting("age");
        assert_eq!(value, None);
    }
    
}