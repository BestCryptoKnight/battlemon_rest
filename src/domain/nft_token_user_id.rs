use std::lazy::SyncLazy;

use regex::Regex;

use crate::domain::Parse;

static RE: SyncLazy<Regex> = SyncLazy::new(|| {
    Regex::new(r#"^(([a-z\d]+[\-_])*[a-z\d]+\.)*([a-z\d]+[\-_])*[a-z\d]+$"#)
        .expect("Couldn't compile regexp expression")
});

impl Parse<String> for UserId {
    fn parse(user_id: Option<String>) -> Result<Option<Self>, String> {
        Self::parse(user_id.as_deref())
    }
}

impl Parse<&str> for UserId {
    fn parse(user_id: Option<&str>) -> Result<Option<Self>, String> {
        match user_id.map(|v| v.trim()) {
            None => Ok(None),
            Some(id) if id.len() < 2 => Err(format!("User id `{id:?}` is too short (min 2)")),
            Some(id) if id.len() > 64 => Err(format!("User id `{id:?}` is too long (max 64)")),
            Some(id) if !RE.is_match(id) => Err(format!("User id `{id:?}` contains wrong chars.")),
            Some(id) => Ok(Some(UserId(id.to_string()))),
        }
    }
}

#[derive(Debug)]
pub struct UserId(String);

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Parse;

    use super::*;

    #[test]
    fn user_id_too_short() {
        let id = Some("a");
        let actual = UserId::parse(id);
        assert!(
            actual.is_err(),
            "The actual `UserId` isn't `Err`, actual value {:?}",
            actual
        );
    }

    #[test]
    fn user_id_too_long() {
        let id = "a".repeat(65);
        let actual = UserId::parse(Some(id));
        assert!(
            actual.is_err(),
            "The actual `UserId` isn't `Err`, actual value {:?}",
            actual
        );
    }

    #[test]
    fn user_id_contains_wrong_chars() {
        let invalid_user_ids = ["fomo@.testnet", "dev-1603749005325-6/432576", "alice;"];
        for id in invalid_user_ids {
            let actual = UserId::parse(Some(id));
            assert!(
                actual.is_err(),
                "The actual `UserId` isn't `Err`, actual value {:?}",
                actual
            );
        }
    }

    #[test]
    fn valid_user_id() {
        let valid_user_ids = [
            "fomo.testnet",
            "fomo.alice.testnet",
            "alice.near",
            "alice.fomo.near",
            "dev-1603749005325-6432576",
        ];
        for id in valid_user_ids {
            let actual = UserId::parse(Some(id));
            assert!(
                actual.is_ok(),
                "The actual `UserId` isn't `Ok`, actual value {:?}",
                actual
            );
        }
    }
}
