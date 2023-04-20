use std::collections::*;
use std::sync::*;
use std::mem::MaybeUninit;

use crate::settings::Settings;
use crate::settings::providers::*;

type SettingsHashSet = HashMap<String, Box<dyn Settings>>;

#[non_exhaustive]
#[derive(Debug)]
pub enum SettingError
{
    InvalidFormat,
}

struct SettingsManager
{
    settings: SettingsHashSet,
    lock: Mutex<u8>
}

impl SettingsManager
{
    pub fn new() -> Self
    {
        SettingsManager 
        { 
            settings: SettingsHashSet::new(), 
            lock: Mutex::new(0)
        }
    }
}

fn get_settings_manager() -> &'static mut SettingsManager
{
    static ONCE: Once = Once::new();
    static mut ALL_SETTINGS: MaybeUninit<SettingsManager> = MaybeUninit::uninit();

    unsafe
    {
        ONCE.call_once(|| 
        {
            let mut settings_manager = SettingsManager::new();
            settings_manager.settings.insert("literal".to_string(), Box::new(LiteralSettings::new()));
            settings_manager.settings.insert("counter".to_string(), Box::new(CounterSettings::new()));

            ALL_SETTINGS.write(settings_manager);
        });

        ALL_SETTINGS.assume_init_mut()
    }
}

pub fn get_setting(namespace: &str, setting: &str) -> Option<String>
{
    let settings_manager = get_settings_manager();
    let _lock = settings_manager.lock.lock().unwrap();

    if let Some(provider) =  settings_manager.settings.get_mut(&namespace.to_lowercase())
    {
        provider.get_setting(setting)
    }
    else 
    {
        None    
    }
}

pub fn register_settings(namespace: &str, settings : Box<dyn Settings>)
{
    let settings_manager = get_settings_manager();
    let _lock = settings_manager.lock.lock().unwrap();

    // If there's already one present then aggregate it
    if let Some(tail) = settings_manager.settings.remove(namespace)
    {
        //let tail = existing.unwrap();
        let cons = ConsSettings::new(settings, tail);

        settings_manager.settings.insert(namespace.to_string(), Box::new(cons));
    }
    else
    {
        settings_manager.settings.insert(namespace.to_string(), settings);
    }
}

pub fn is_registered(namespace: &str) -> bool
{
    let settings_manager = get_settings_manager();
    let _lock = settings_manager.lock.lock().unwrap();

    settings_manager.settings.contains_key(namespace)
}

pub fn crack_qualified_name(name: &str) -> Result<(&str, &str), SettingError>
{
    let parts = name.split_once(':');

    match parts
    {
        Some((namespace, setting)) => Ok((namespace, setting)),
        None                       => Err(SettingError::InvalidFormat)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::settings::providers::SingleSettings;

    #[test]
    fn test_is_registered() 
    {
        assert_eq!(is_registered("literal"), true);
        assert_eq!(is_registered("foobar"), false);
    }
    
    #[test]
    fn combine_namespaces() 
    {
        let age = SingleSettings::new("age", "42");
        register_settings("island", Box::new(age));

        assert_eq!(is_registered("island"), true);
        assert_eq!(get_setting("island", "age").unwrap(), "42".to_string());

        let name = SingleSettings::new("name", "Jack");
        register_settings("island", Box::new(name));

        assert_eq!(is_registered("island"), true);
        assert_eq!(get_setting("island", "name").unwrap(), "Jack".to_string());
        assert_eq!(get_setting("island", "age").unwrap(), "42".to_string());
    }

    #[test]
    fn crack() 
    {
        assert!(crack_qualified_name("foo:bar").is_ok());
        assert_eq!(crack_qualified_name("foo:bar").unwrap(), ("foo", "bar"));
        assert_eq!(crack_qualified_name("foo:bar:xyz").unwrap(), ("foo", "bar:xyz"));
    }

    #[test]
    fn crack_bad_name() 
    {
        assert!(crack_qualified_name("foo").is_err());
        assert!(crack_qualified_name("foo?bar").is_err());
    }
}