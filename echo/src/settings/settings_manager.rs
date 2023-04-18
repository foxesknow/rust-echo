use std::collections::*;
use std::sync::*;
use std::mem::MaybeUninit;

use crate::settings::Settings;
use crate::settings::providers::*;

type SettingsHashSet = HashMap<String, Box<dyn Settings>>;

fn get_settings_hashset() -> &'static mut SettingsHashSet
{
    static ONCE: Once = Once::new();
    static mut ALL_SETTINGS: MaybeUninit<SettingsHashSet> = MaybeUninit::uninit();

    unsafe
    {
        ONCE.call_once(|| 
        {
            let mut settings = SettingsHashSet::new();
            settings.insert("literal".to_string(), Box::new(LiteralSettings::new()));
            settings.insert("counter".to_string(), Box::new(CounterSettings::new()));

            ALL_SETTINGS.write(settings);
        });

        ALL_SETTINGS.assume_init_mut()
    }
}

pub fn get_setting(namespace: &str, setting: &str) -> Option<String>
{
    let settings = get_settings_hashset();
    let namespace_manager = settings.get_mut(&namespace.to_lowercase());

    if namespace_manager.is_some()
    {
        let provider = namespace_manager.unwrap();
        provider.get_setting(setting)
    }
    else 
    {
        None    
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_test_found() 
    {
        let name = get_setting("literal", "ben");
        assert_eq!(name.unwrap(), "ben");
    }

    #[test]
    fn literal_test_not() 
    {
        let name = get_setting("foo", "ben");
        assert_eq!(name, None);
    }
}