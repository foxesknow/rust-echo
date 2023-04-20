use crate::settings::Settings;

pub struct FirstSettings
{
}

impl FirstSettings
{
    pub fn new() -> Self
    {
        FirstSettings{}
    }
}

impl Settings for FirstSettings
{
    fn get_setting(&mut self, name : &str) -> Option<String>
    {
        let _parts = name.split("??");

        None
    }
}