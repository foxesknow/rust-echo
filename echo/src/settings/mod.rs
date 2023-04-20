pub mod providers;
pub mod settings_manager;
pub mod expander;

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum SettingError
{
    NotFound,
    InvalidFormat,
    NotTerminated,
    UnknownAction(String),
}

pub trait Settings: Send + Sync
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>;
}