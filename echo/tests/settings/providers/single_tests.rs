use echo::settings::*;
use echo::settings::providers::SingleSettings;

#[test]
fn has_value() -> Result<(), SettingError>
{
    let mut single = SingleSettings::new("name", "Jack");
    let value = single.get_setting("name");
    assert_eq!(value?, "Jack".to_string());

    Ok(())
}

#[test]
fn has_value_case() -> Result<(), SettingError>
{
    let mut single = SingleSettings::new("name", "Jack");
    let value = single.get_setting("NAME");
    assert_eq!(value?, "Jack".to_string());

    Ok(())
}

#[test]
fn no_value() -> Result<(), SettingError>
{
    let mut single = SingleSettings::new("name", "Jack");
    let value = single.get_setting("age");
    assert_eq!(value.unwrap_err(), SettingError::NotFound);

    Ok(())
}
