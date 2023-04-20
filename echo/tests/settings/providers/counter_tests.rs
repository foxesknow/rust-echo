use echo::settings::*;
use echo::settings::providers::CounterSettings;

#[test]
fn starts_with_1() {
    let mut counter = CounterSettings::new();
    let value1 = counter.get_setting("next");
    assert_eq!(value1.unwrap(), "1".to_string());
}

#[test]
fn next_returns_different_value() {
    let mut counter = CounterSettings::new();
    let value1 = counter.get_setting("next").unwrap();
    assert_eq!(value1, "1".to_string());

    let value2 = counter.get_setting("next").unwrap();
    assert_ne!(value1, value2);
}