use crate::settings::Settings;

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
    fn get_setting(&mut self, name : &str) -> Option<String>
    {
        Some(name.to_string())
    }
}