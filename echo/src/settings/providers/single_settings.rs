use crate::settings::Settings;

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
    fn get_setting(&mut self, name : &str) -> Option<String>
    {
        if self.name == name.to_lowercase()
        {
            Some(self.value.to_string())
        }
        else
        {
            None
        }
    }
}