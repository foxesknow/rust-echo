use echo::settings::SettingError;
use echo::settings::settings_manager::*;
use echo::settings::providers::*;

#[test]
fn test_is_registered() 
{
    assert_eq!(is_registered("literal"), true);
    assert_eq!(is_registered("foobar"), false);
}

#[test]
fn combine_namespaces() -> Result<(), SettingError>
{
    let age = SingleSettings::new("age", "42");
    register_settings("island", Box::new(age));

    assert_eq!(is_registered("island"), true);
    assert_eq!(get_setting("island", "age")?, "42".to_string());

    let name = SingleSettings::new("name", "Jack");
    register_settings("island", Box::new(name));

    assert_eq!(is_registered("island"), true);
    assert_eq!(get_setting("island", "name")?, "Jack".to_string());
    assert_eq!(get_setting("island", "age")?, "42".to_string());

    Ok(())
}

#[test]
fn crack() -> Result<(), SettingError>
{
    assert!(crack_qualified_name("foo:bar").is_ok());
    assert_eq!(crack_qualified_name("foo:bar")?, ("foo", "bar"));
    assert_eq!(crack_qualified_name("foo:bar:xyz")?, ("foo", "bar:xyz"));

    Ok(())
}

#[test]
fn crack_bad_name() -> Result<(), SettingError>
{
    assert!(crack_qualified_name("foo").is_err());
    assert!(crack_qualified_name("foo?bar").is_err());

    Ok(())
}