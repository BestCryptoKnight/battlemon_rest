use crate::domain::Parse;

#[derive(Debug, Clone)]
pub struct TokenId(Option<String>);

crate::domain::impl_into_inner!(TokenId);
crate::domain::impl_as_inner!(TokenId);

impl Parse<&str> for TokenId {
    fn parse(token_id: Option<&str>) -> Result<Self, String> {
        match token_id.map(|v| v.trim()) {
            None => Ok(TokenId(None)),
            Some(id) if id.is_empty() => Err("Token id is empty".to_string()),
            Some(id) if !id.chars().all(|ch| ch.is_ascii_digit()) => {
                Err("The token id must contain only digits".to_string())
            }
            Some(id) => Ok(TokenId(Some(id.to_string()))),
        }
    }
}

impl Parse<String> for TokenId {
    fn parse(token_id: Option<String>) -> Result<Self, String> {
        Self::parse(token_id.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::IntoInner;
    use rand::prelude::StdRng;
    use rand::{Rng, SeedableRng};

    use super::*;

    #[derive(Debug, Clone)]
    pub struct ValidTokenIdFixture(pub Option<String>);

    impl quickcheck::Arbitrary for ValidTokenIdFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let token_id: u64 = rng.gen();
            Self(Some(token_id.to_string()))
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_token_ids(valid_token_id: ValidTokenIdFixture) -> bool {
        TokenId::parse(valid_token_id.0).is_ok()
    }

    #[test]
    fn user_id_contains_wrong_chars() {
        let invalid_token_ids = ["a", "1a", "a1", "123a", "12.", ".12", "+12", "-12", "1+2"];
        for id in invalid_token_ids {
            let actual = TokenId::parse(Some(id));
            assert!(
                actual.is_err(),
                "The actual `TokenId` isn't `Err`, actual value {:?}",
                actual
            );
        }
    }

    #[test]
    fn empty_token_id_is_rejected() {
        let id = Some(String::new());
        let actual = TokenId::parse(id);
        assert!(
            actual.is_err(),
            "The actual `TokenId` isn't `Err`, actual value {:?}",
            actual
        );
    }

    #[test]
    fn when_token_id_is_none_then_parsed_is_ok_none() {
        let id: Option<String> = None;
        let actual = TokenId::parse(id);
        assert!(
            actual.is_ok(),
            "The actual `TokenId` isn't `Ok`, actual value {:?}",
            actual
        );
        let inner_actual = actual.unwrap().into_inner();
        assert!(
            inner_actual.is_none(),
            "The actual inner of `TokenId` isn't `None`, actual inner value {:?}",
            inner_actual
        )
    }
}
