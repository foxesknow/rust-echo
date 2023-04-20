use crate::settings::*;

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
        let parts = name.split("??");

        for part in parts
        {
            let setting_name = part.trim();

            if let Ok((namespace, _)) = settings_manager::crack_qualified_name(setting_name)
            {
                if settings_manager::is_registered(namespace)
                {
                    match expander::expand_token(setting_name)
                    {
                        Ok(expanded_token)          => return Ok(expanded_token),
                        Err(SettingError::NotFound) => continue,
                        Err(x)                      => return Err(x)
                    }
                }
            }
        }
        Err(SettingError::NotFound)
    }
}