use crate::{
    api::auth::universal_auth_login::universal_auth_login, auth::AccessTokenSuccessResponse,
    client::auth_method_settings::UniversalAuthMethod, error::Result, Client,
};

pub async fn universal_auth(
    client: &mut Client,
    input: &UniversalAuthMethod,
) -> Result<AccessTokenSuccessResponse> {
    return universal_auth_login(client, input.client_id.clone(), input.client_secret.clone())
        .await;
}
