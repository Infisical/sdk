use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    access_token::{access_token_request, AccessTokenSuccessResponse},
    error::Result,
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateAccessTokenRequest; // No input. But we have to do this to get a schema for the request.

pub async fn update_access_token(client: &mut Client) -> Result<AccessTokenSuccessResponse> {
    let res = access_token_request(client).await;

    // If the response is ok, then we set the client access token, otherwise we throw an error
    match res {
        Ok(res) => {
            client.set_access_token(res.access_token.clone());
            Ok(res)
        }
        Err(res) => Err(res),
    }
}
