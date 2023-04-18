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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_value() 
    {
        let mut single = SingleSettings::new("name", "Jack");
        let value = single.get_setting("name");
        assert_eq!(value.unwrap(), "Jack".to_string());
    }

    #[test]
    fn has_value_case() 
    {
        let mut single = SingleSettings::new("name", "Jack");
        let value = single.get_setting("NAME");
        assert_eq!(value.unwrap(), "Jack".to_string());
    }

   #[test]
    fn no_value() 
    {
        let mut single = SingleSettings::new("name", "Jack");
        let value = single.get_setting("age");
        assert_eq!(value, None);
    }
}
