use crate::settings::*;

static TOKEN_PIPE_SEPARATOR: char = '|';

pub fn expand_text(text: &str, begin_token: &str, end_token: &str) -> Result<String, &'static str>
{
    return expand_text_with_lookup(text, begin_token, end_token, None)
}

pub fn expand_text_with_lookup(text: &str, begin_token: &str, end_token: &str, unknown_variable_lookup : Option<fn(&str) -> Option<String>>) -> Result<String, &'static str>
{
    let mut expanded = String::with_capacity(text.len());
    let mut parse_buffer = text;

    while let Some(begin) = parse_buffer.find(begin_token)
    {
        expanded.push_str(&parse_buffer[0..begin]);
        parse_buffer = &parse_buffer[begin + begin_token.len()..];

        if let Some(end) = parse_buffer.find(end_token)
        {
            let token = &parse_buffer[0..end];
            parse_buffer = &parse_buffer[end + end_token.len()..];

            let token_value = expand_token_with_lookup(token, unknown_variable_lookup)?;
            expanded.push_str(&token_value);
        }
        else
        {
            return Err("token not terminated");
        }
    }

    // Add on whatever is left
    expanded.push_str(parse_buffer);

    Ok(expanded)
}

pub fn expand_token(token: &str) -> Result<String, &'static str>
{
    expand_token_with_lookup(token, None)
}

pub fn expand_token_with_lookup(token: &str, unknown_variable_lookup : Option<fn(&str) -> Option<String>>) -> Result<String, &'static str>
{
    let mut parts = token.splitn(2, TOKEN_PIPE_SEPARATOR);
    
    let variable = parts.next().ok_or("no variable")?;
    let action = parts.next();
    
    let (namespace, setting) = extract_namespace_and_setting(variable);

    let value = if namespace.is_some()
    {
        settings_manager::get_setting(namespace.unwrap(), setting.unwrap())
    }
    else
    {
        match unknown_variable_lookup
        {
            Some(function) => function(variable),
            None => None
        }
    };

    if value.is_none() 
    {
        return Err("setting not found");
    }

    let result = match action
    {
        Some("trim") => value.unwrap().trim().to_string(),
        Some("trim_end") => value.unwrap().trim_end().to_string(),
        Some("trim_start") => value.unwrap().trim_start().to_string(),
        Some("to_upper") => value.unwrap().to_uppercase().to_string(),
        Some("to_lower") => value.unwrap().to_lowercase().to_string(),
        _ => value.unwrap()
    };

    Ok(result)
}

fn extract_namespace_and_setting(variable: &str) ->(Option<&str>, Option<&str>)
{
    let mut parts = variable.splitn(2, ':');

    let first = parts.next();
    let second = parts.next();

    if second.is_some()
    {
        (first, second)
    }
    else
    {
        (None, second)    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() 
    {
        assert!(expand_token("hello:world").is_err());
    }

    #[test]
    fn test_literal() -> Result<(), String>
    {
        assert_eq!(expand_token("literal:hello")?, "hello".to_string());
        Ok(())
    }

    #[test]
    fn test_literal_to_upper() -> Result<(), String>
    {
        assert_eq!(expand_token("literal:hello|to_upper")?, "HELLO".to_string());
        Ok(())
    }

    #[test]
    fn test_literal_to_lower() -> Result<(), String>
    {
        assert_eq!(expand_token("literal:Hello|to_lower")?, "hello".to_string());
        Ok(())
    }

    #[test]
    fn test_expand_text_one_token_1() -> Result<(), String>
    {
        assert_eq!(expand_text("hello ${literal:bob}", "${", "}")?, "hello bob".to_string());
        Ok(())
    }

    #[test]
    fn test_expand_text_one_token_2() -> Result<(), String>
    {
        assert_eq!(expand_text("AB ${literal:bob} CD", "${", "}")?, "AB bob CD".to_string());
        Ok(())
    }

    #[test]
    fn test_expand_text_two_tokens_1() -> Result<(), String>
    {
        assert_eq!(expand_text("hello ${literal:Bob} how is ${literal:Jack} doing", "${", "}")?, "hello Bob how is Jack doing".to_string());
        Ok(())
    }
}