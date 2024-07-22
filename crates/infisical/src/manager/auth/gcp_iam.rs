use crate::{
    api::auth::gcp_iam_login::gcp_iam_login, auth::AccessTokenSuccessResponse,
    client::auth_method_settings::GCPIamAuthMethod, error::Result, Client,
};

pub async fn gcp_iam(
    client: &mut Client,
    input: &GCPIamAuthMethod,
) -> Result<AccessTokenSuccessResponse> {
    return gcp_iam_login(
        client,
        input.identity_id.clone(),
        input.service_account_key_file_path.clone(),
    )
    .await;
}
