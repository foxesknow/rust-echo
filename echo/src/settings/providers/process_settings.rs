use crate::settings::*;

pub struct ProcessSettings
{
}

impl ProcessSettings
{
    pub fn new() -> Self
    {
        ProcessSettings{}
    }

    fn get_exe(&self) -> Result<String, SettingError>
    {
        let exe = std::env::current_exe().map_err(|_| SettingError::NotFound)?;
        
        match exe.file_name()
        {
            Some(filename)  =>  match filename.to_str()
                                {
                                    Some(s) => Ok(s.to_string()),
                                    _       => Err(SettingError::NotFound)
                                }
            _error          => Err(SettingError::NotFound)
        }
    }

    fn get_path(&self) -> Result<String, SettingError>
    {
        let exe = std::env::current_exe().map_err(|_| SettingError::NotFound)?;

        match exe.parent()
        {
            Some(path) =>   match path.to_str()
                            {
                                Some(path) => Ok(path.to_string()),
                                _          => Err(SettingError::NotFound)
                            }
            _          => Err(SettingError::NotFound)
        }
    }
}

impl Settings for ProcessSettings
{
    fn get_setting(&mut self, name : &str) -> Result<String, SettingError>
    {
        match name.to_lowercase().as_str()
        {
            "exe"        => self.get_exe(),
            "startupdir" => self.get_path(),
            _            => Err(SettingError::NotFound)
        }

        
    }    
}
