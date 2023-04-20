use crate::settings::Settings;
use crate::settings::settings_manager;
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
    fn get_setting(&mut self, name : &str) -> Option<String>
    {
        /*
        let parts = name.split("??");

        for part in parts
        {
            settings_manager::crack_qualified_name(name)
        }
*/
        None
    }
}