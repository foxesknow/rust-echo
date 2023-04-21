use echo::settings::*;
use echo::settings::providers::ProcessSettings;

#[test]
fn get_startupdir() -> Result<(), SettingError>
{
    let mut settings = ProcessSettings::new();
    let value = settings.get_setting("startupdir");
    assert!(value.is_ok());

    Ok(())
}

#[test]
fn get_exe() -> Result<(), SettingError>
{
    let mut settings = ProcessSettings::new();
    let value = settings.get_setting("exe");
    assert!(value.is_ok());

    Ok(())
}
