use crate::{
    api::auth::azure_login::azure_login, auth::AccessTokenSuccessResponse,
    client::auth_method_settings::AzureAuthMethod, error::Result, Client,
};

pub async fn azure_auth(
    client: &mut Client,
    input: &AzureAuthMethod,
) -> Result<AccessTokenSuccessResponse> {
    return azure_login(client, input.identity_id.clone()).await;
}
