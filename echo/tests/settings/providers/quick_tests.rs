use echo::settings::*;
use echo::settings::providers::QuickSettings;

#[test]
fn initialize() -> Result<(), SettingError>
{
    let values = [("name", "Jack")];
    let mut quick = QuickSettings::new(&values);

    let value = quick.get_setting("name");
    assert_eq!(value?, "Jack".to_string());

    Ok(())
}

#[test]
fn no_such_value() -> Result<(), SettingError>
{
    let values = [("name", "Jack")];
    let mut quick = QuickSettings::new(&values);

    let value = quick.get_setting("age");
    assert_eq!(value.unwrap_err(), SettingError::NotFound);

    Ok(())
}