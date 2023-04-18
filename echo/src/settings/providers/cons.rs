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

#[cfg(test)]
mod tests {
    use crate::settings::providers::SingleSettings;

    use super::*;

    #[test]
    fn initialization() {
        let head = SingleSettings::new("name", "Jack");
        let tail = SingleSettings::new("age", "42");
        
        let mut cons = ConsSettings::new(Box::new(head), Box::new(tail));
        
        let name = cons.get_setting("name");
        assert_eq!(name.unwrap(), "Jack".to_string());
        
        let age = cons.get_setting("age");
        assert_eq!(age.unwrap(), "42".to_string());
    }
}
