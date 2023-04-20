use crate::settings::*;

pub struct ConsSettings
{
    head: Box<dyn Settings>,
    tail: Box<dyn Settings>
}

impl ConsSettings
{
    pub fn new(head: Box<dyn Settings>, tail: Box<dyn Settings>) -> Self
    {
        ConsSettings
        {
            head: head,
            tail: tail
        }
    }
}

impl Settings for ConsSettings
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>
    {
        match self.head.get_setting(name)
        {
            Ok(x) => Ok(x),
            _     => self.tail.get_setting(name)
        }
    }
}


