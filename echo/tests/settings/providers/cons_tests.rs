use echo::settings::*;
use echo::settings::providers::SingleSettings;
use echo::settings::providers::ConsSettings;

#[test]
fn initialization() {
    let head = SingleSettings::new("name", "Jack");
    let tail = SingleSettings::new("age", "42");
    
    let mut cons = ConsSettings::new(Box::new(head), Box::new(tail));
    
    let name = cons.get_setting("name");
    assert_eq!(name.unwrap(), "Jack".to_string());
    
    let age = cons.get_setting("age");
    assert_eq!(age.unwrap(), "42".to_string());
}
