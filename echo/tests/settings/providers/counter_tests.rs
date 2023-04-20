use echo::settings::*;
use echo::settings::providers::CounterSettings;

#[test]
fn starts_with_1() -> Result<(), SettingError>
{
    let mut counter = CounterSettings::new();
    let value1 = counter.get_setting("next");
    assert_eq!(value1?, "1".to_string());

    Ok(())
}

#[test]
fn next_returns_different_value() -> Result<(), SettingError>
{
    let mut counter = CounterSettings::new();
    let value1 = counter.get_setting("next")?;
    assert_eq!(value1, "1".to_string());

    let value2 = counter.get_setting("next")?;
    assert_ne!(value1, value2);

    Ok(())
}