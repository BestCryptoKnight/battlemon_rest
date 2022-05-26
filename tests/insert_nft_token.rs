use crate::helpers::spawn_app;

use crate::dummies::AliceNftToken;
use fake::Fake;

mod dummies;
mod helpers;

#[tokio::test]
async fn insert_valid_nft_token_success() {
    let app = spawn_app().await;
    let token: dummies::NftToken = AliceNftToken.fake();
    let response = app.post_nft_token(&token).await;

    let status = response.status();
    let body = response
        .text()
        .await
        .expect("Couldn't get the response text");

    assert_eq!(
        status, 201,
        "The expected response doesn't have `201` status code.\
         The actual response has status code `{}` and body: `{}`",
        status, body,
    );
}
