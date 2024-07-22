use crate::{
    api::auth::kubernetes_login::kubernetes_login, auth::AccessTokenSuccessResponse,
    client::auth_method_settings::KubernetesAuthMethod, error::Result, Client,
};

pub async fn kubernetes_auth(
    client: &mut Client,
    input: &KubernetesAuthMethod,
) -> Result<AccessTokenSuccessResponse> {
    return kubernetes_login(
        client,
        input.identity_id.clone(),
        input
            .service_account_token_path
            .clone()
            .unwrap_or("/var/run/secrets/kubernetes.io/serviceaccount/token".to_string()),
    )
    .await;
}
