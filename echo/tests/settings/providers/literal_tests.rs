use echo::settings::*;
use echo::settings::providers::LiteralSettings;

#[test]
fn literal_test() -> Result<(), SettingError>
{
    let mut s = LiteralSettings::new();
    let value = s.get_setting("foo");
    assert_eq!(value?, "foo");

    Ok(())
}