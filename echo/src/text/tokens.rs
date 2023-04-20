use crate::settings::*;

static TOKEN_PIPE_SEPARATOR: char = '|';

#[non_exhaustive]
#[derive(Debug)]
pub enum ExpansionError
{
    NotTerminated,
    NoVariable,
    SettingNotFound,
    UnknownAction(String)
}

pub fn expand_text(text: &str, begin_token: &str, end_token: &str) -> Result<String, ExpansionError>
{
    return expand_text_with_lookup(text, begin_token, end_token, None)
}

pub fn expand_text_with_lookup(text: &str, begin_token: &str, end_token: &str, unknown_variable_lookup : Option<fn(&str) -> Option<String>>) -> Result<String, ExpansionError>
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
            return Err(ExpansionError::NotTerminated);
        }
    }

    // Add on whatever is left
    expanded.push_str(parse_buffer);

    Ok(expanded)
}

pub fn expand_token(token: &str) -> Result<String, ExpansionError>
{
    expand_token_with_lookup(token, None)
}

pub fn expand_token_with_lookup(token: &str, unknown_variable_lookup : Option<fn(&str) -> Option<String>>) -> Result<String, ExpansionError>
{
    let mut parts = token.splitn(2, TOKEN_PIPE_SEPARATOR);
    
    let variable = parts.next().ok_or(ExpansionError::NoVariable)?;
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
        return Err(ExpansionError::SettingNotFound);
    }

    let result = match action
    {
        Some("trim")        => value.unwrap().trim().to_string(),
        Some("trim_end")    => value.unwrap().trim_end().to_string(),
        Some("trim_start")  => value.unwrap().trim_start().to_string(),
        Some("to_upper")    => value.unwrap().to_uppercase().to_string(),
        Some("to_lower")    => value.unwrap().to_lowercase().to_string(),
        Some(x)             => return Err(ExpansionError::UnknownAction(x.to_string())),
        _                   => value.unwrap()
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