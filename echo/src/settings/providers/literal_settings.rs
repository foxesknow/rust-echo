use crate::settings::*;

pub struct LiteralSettings
{
}

impl LiteralSettings
{
    pub fn new() -> Self
    {
        LiteralSettings{}
    }
}

impl Settings for LiteralSettings
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>
    {
        Ok(name.to_string())
    }
}