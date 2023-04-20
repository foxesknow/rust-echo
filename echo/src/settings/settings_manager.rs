use std::collections::*;
use std::sync::*;
use std::mem::MaybeUninit;

use crate::settings::*;
use crate::settings::providers::*;

type SettingsHashSet = HashMap<String, Box<dyn Settings>>;

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

pub fn get_setting(namespace: &str, setting: &str) -> Result<String, SettingError>
{
    let settings_manager = get_settings_manager();
    let _lock = settings_manager.lock.lock().unwrap();

    if let Some(provider) =  settings_manager.settings.get_mut(&namespace.to_lowercase())
    {
        provider.get_setting(setting)
    }
    else 
    {
        Err(SettingError::NotFound)    
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