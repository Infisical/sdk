use crate::{
    api::auth::aws_iam_login::aws_iam_login, auth::AccessTokenSuccessResponse,
    client::auth_method_settings::AWSIamAuthMethod, error::Result, Client,
};

pub async fn aws_iam(
    client: &mut Client,
    input: &AWSIamAuthMethod,
) -> Result<AccessTokenSuccessResponse> {
    return aws_iam_login(client, input.identity_id.clone()).await;
}
