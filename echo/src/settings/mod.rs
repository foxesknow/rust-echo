pub mod providers;
pub mod settings_manager;

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum SettingError
{
    NotFound,
    InvalidFormat,
}

pub trait Settings: Send + Sync
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>;
}