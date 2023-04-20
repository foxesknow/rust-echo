use echo::settings::*;
use echo::settings::providers::SingleSettings;

#[test]
fn has_value() 
{
    let mut single = SingleSettings::new("name", "Jack");
    let value = single.get_setting("name");
    assert_eq!(value.unwrap(), "Jack".to_string());
}

#[test]
fn has_value_case() 
{
    let mut single = SingleSettings::new("name", "Jack");
    let value = single.get_setting("NAME");
    assert_eq!(value.unwrap(), "Jack".to_string());
}

#[test]
fn no_value() 
{
    let mut single = SingleSettings::new("name", "Jack");
    let value = single.get_setting("age");
    assert_eq!(value, None);
}
