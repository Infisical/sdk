use crate::{
    api::auth::gcp_id_token_login::gcp_id_token_login, auth::AccessTokenSuccessResponse,
    client::auth_method_settings::GCPIdTokenAuthMethod, error::Result, Client,
};

pub async fn gcp_id_token(
    client: &mut Client,
    input: &GCPIdTokenAuthMethod,
) -> Result<AccessTokenSuccessResponse> {
    return gcp_id_token_login(client, input.identity_id.clone()).await;
}
