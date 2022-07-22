use once_cell::sync::Lazy;

use regex::Regex;

use crate::domain::Parse;

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^(([a-z\d]+[\-_])*[a-z\d]+\.)*([a-z\d]+[\-_])*[a-z\d]+$"#)
        .expect("Couldn't compile regexp expression")
});

#[derive(Debug)]
pub struct NftTokenOwnerId(Option<String>);

crate::domain::impl_into_inner!(NftTokenOwnerId);
crate::domain::impl_as_inner!(NftTokenOwnerId);

impl Parse<String> for NftTokenOwnerId {
    fn parse(owner_id: Option<String>) -> Result<Self, String> {
        Self::parse(owner_id.as_deref())
    }
}

impl Parse<&str> for NftTokenOwnerId {
    fn parse(owner_id: Option<&str>) -> Result<Self, String> {
        match owner_id.map(|v| v.trim()) {
            None => Ok(NftTokenOwnerId(None)),
            Some(id) if id.len() < 2 => Err(format!("User id `{id:?}` is too short (min 2)")),
            Some(id) if id.len() > 64 => Err(format!("User id `{id:?}` is too long (max 64)")),
            Some(id) if !RE.is_match(id) => Err(format!("User id `{id:?}` contains wrong chars.")),
            Some(id) => Ok(NftTokenOwnerId(Some(id.to_string()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{IntoInner, Parse};

    use super::*;

    #[test]
    fn owner_id_too_short() {
        let id = Some("a");
        let actual = NftTokenOwnerId::parse(id);
        assert!(
            actual.is_err(),
            "The actual `UserId` isn't `Err`, actual value {:?}",
            actual
        );
    }

    #[test]
    fn owner_id_too_long() {
        let id = "a".repeat(65);
        let actual = NftTokenOwnerId::parse(Some(id));
        assert!(
            actual.is_err(),
            "The actual `UserId` isn't `Err`, actual value {:?}",
            actual
        );
    }

    #[test]
    fn owner_id_contains_wrong_chars() {
        let invalid_owner_ids = ["fomo@.testnet", "dev-1603749005325-6/432576", "alice;"];
        for id in invalid_owner_ids {
            let actual = NftTokenOwnerId::parse(Some(id));
            assert!(
                actual.is_err(),
                "The actual `UserId` isn't `Err`, actual value {:?}",
                actual
            );
        }
    }

    #[test]
    fn valid_owner_id() {
        let valid_owner_ids = [
            "fomo.testnet",
            "fomo.alice.testnet",
            "alice.near",
            "alice.fomo.near",
            "dev-1603749005325-6432576",
        ];
        for id in valid_owner_ids {
            let actual = NftTokenOwnerId::parse(Some(id));
            assert!(
                actual.is_ok(),
                "The actual `UserId` isn't `Ok`, actual value {:?}",
                actual
            );
        }
    }

    #[test]
    fn none_is_ok_none() {
        let token_id: Option<String> = None;
        let actual = NftTokenOwnerId::parse(token_id);
        assert!(
            actual.is_ok(),
            "The actual `UserId` isn't `Ok`, actual value {:?}",
            actual
        );
        let inner_actual = actual.unwrap().into_inner();
        assert!(
            inner_actual.is_none(),
            "The actual inner of `UserId` isn't `None`, actual inner value {:?}",
            inner_actual
        );
    }
}
