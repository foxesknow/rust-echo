use echo::settings::*;
use echo::settings::providers::QuickSettings;

#[test]
fn initialize() 
{
    let values = [("name", "Jack")];
    let mut quick = QuickSettings::new(&values);

    let value = quick.get_setting("name");
    assert_eq!(value.unwrap(), "Jack".to_string());
}

#[test]
fn no_such_value() 
{
    let values = [("name", "Jack")];
    let mut quick = QuickSettings::new(&values);

    let value = quick.get_setting("age");
    assert_eq!(value, None);
}