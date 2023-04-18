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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_test() {
        let mut s = LiteralSettings::new();
        let value = s.get_setting("foo");
        assert_eq!(value.unwrap(), "foo");
    }
}