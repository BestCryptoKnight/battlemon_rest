use crate::domain::Parse;

#[derive(Debug)]
pub struct TokenId(String);

impl AsRef<str> for TokenId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Parse<&str> for TokenId {
    fn parse(token_id: Option<&str>) -> Result<Option<Self>, String> {
        match token_id.map(|v| v.trim()) {
            None => Ok(None),
            Some(id) if id.is_empty() => Err("Token id is empty".to_string()),
            Some(id) if !id.chars().all(|ch| ch.is_ascii_digit()) => {
                Err("The token id must contain only digits".to_string())
            }
            Some(id) => Ok(Some(TokenId(id.to_string()))),
        }
    }
}

impl Parse<String> for TokenId {
    fn parse(token_id: Option<String>) -> Result<Option<Self>, String> {
        Self::parse(token_id.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_token_id_is_none_then_parsed_is_ok_none() {
        todo!()
    }
}
