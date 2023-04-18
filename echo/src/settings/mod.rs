pub mod providers;
pub mod settings_manager;

pub trait Settings: Send + Sync
{
    fn get_setting(&mut self, name : &str) -> Option<String>;
}