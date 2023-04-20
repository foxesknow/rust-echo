use echo::settings::*;
use echo::settings::expander::*;

#[test]
fn test_token() 
{
    assert!(expand_token("hello:world").is_err());
}

#[test]
fn test_literal() -> Result<(), SettingError>
{
    assert_eq!(expand_token("literal:hello")?, "hello".to_string());
    Ok(())
}

#[test]
fn test_literal_to_upper() -> Result<(), SettingError>
{
    assert_eq!(expand_token("literal:hello|to_upper")?, "HELLO".to_string());
    Ok(())
}

#[test]
fn test_literal_to_lower() -> Result<(), SettingError>
{
    assert_eq!(expand_token("literal:Hello|to_lower")?, "hello".to_string());
    Ok(())
}

#[test]
fn test_expand_text_one_token_1() -> Result<(), SettingError>
{
    assert_eq!(expand_text("hello ${literal:bob}", "${", "}")?, "hello bob".to_string());
    Ok(())
}

#[test]
fn test_expand_text_one_token_2() -> Result<(), SettingError>
{
    assert_eq!(expand_text("AB ${literal:bob} CD", "${", "}")?, "AB bob CD".to_string());
    Ok(())
}

#[test]
fn test_expand_text_two_tokens_1() -> Result<(), SettingError>
{
    assert_eq!(expand_text("hello ${literal:Bob} how is ${literal:Jack} doing", "${", "}")?, "hello Bob how is Jack doing".to_string());
    Ok(())
}