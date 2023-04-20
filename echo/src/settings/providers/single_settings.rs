use crate::settings::*;

pub struct SingleSettings
{
    name: String,
    value: String
}

impl SingleSettings
{
    pub fn new(name: &str, value: &str) -> Self
    {
        return SingleSettings
        {
            name: name.to_lowercase(),
            value: value.to_string()
        }
    }
}

impl Settings for SingleSettings
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>
    {
        if self.name == name.to_lowercase()
        {
            Ok(self.value.to_string())
        }
        else
        {
            Err(SettingError::NotFound)
        }
    }
}