use crate::settings::Settings;

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
    fn get_setting(&mut self, name : &str) -> Option<String>
    {
        match self.head.get_setting(name)
        {
            Some(x) => Some(x),
            _        => self.tail.get_setting(name)
        }
    }
}


