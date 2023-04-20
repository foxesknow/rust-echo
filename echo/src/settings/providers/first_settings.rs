use crate::settings::*;
use crate::text::tokens;

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
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>
    {
        /*
        let parts = name.split("??");

        for part in parts
        {
            settings_manager::crack_qualified_name(name)
        }
*/
        Err(SettingError::NotFound)
    }
}