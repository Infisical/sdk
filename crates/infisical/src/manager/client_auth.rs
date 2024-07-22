use crate::{
    auth::AccessTokenSuccessResponse,
    client::auth_method_settings::{
        AWSIamAuthMethod, AzureAuthMethod, GCPIamAuthMethod, GCPIdTokenAuthMethod,
        KubernetesAuthMethod, UniversalAuthMethod,
    },
    error::Result,
    Client,
};

use super::auth::{
    aws_iam::aws_iam, azure::azure_auth, gcp_iam::gcp_iam, gcp_id_token::gcp_id_token,
    kubernetes::kubernetes_auth, universal_auth::universal_auth,
};

#[allow(dead_code)]
pub struct ClientAuth<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientAuth<'a> {
    fn handle_auth_response(&'a mut self, response: &Result<AccessTokenSuccessResponse>) {
        if response.is_ok() {
            let response = response.as_ref().unwrap();
            self.client.set_access_token(response.access_token.clone());
        }
    }

    pub async fn universal_login(
        &'a mut self,
        input: &UniversalAuthMethod,
    ) -> Result<AccessTokenSuccessResponse> {
        let response = universal_auth(self.client, input).await;

        self.handle_auth_response(&response);
        return response;
    }

    pub async fn kubernetes_login(
        &'a mut self,
        input: &KubernetesAuthMethod,
    ) -> Result<AccessTokenSuccessResponse> {
        let response = kubernetes_auth(self.client, input).await;

        self.handle_auth_response(&response);
        return response;
    }

    pub async fn azure_login(
        &'a mut self,
        input: &AzureAuthMethod,
    ) -> Result<AccessTokenSuccessResponse> {
        let response = azure_auth(self.client, input).await;

        self.handle_auth_response(&response);
        return response;
    }

    pub async fn gcp_id_token_login(
        &'a mut self,
        input: &GCPIdTokenAuthMethod,
    ) -> Result<AccessTokenSuccessResponse> {
        let response = gcp_id_token(self.client, input).await;

        self.handle_auth_response(&response);
        return response;
    }

    pub async fn gcp_iam_login(
        &'a mut self,
        input: &GCPIamAuthMethod,
    ) -> Result<AccessTokenSuccessResponse> {
        let response = gcp_iam(self.client, input).await;

        self.handle_auth_response(&response);
        return response;
    }

    pub async fn aws_iam_login(
        &'a mut self,
        input: &AWSIamAuthMethod,
    ) -> Result<AccessTokenSuccessResponse> {
        let response = aws_iam(self.client, input).await;

        self.handle_auth_response(&response);
        return response;
    }
}

impl<'a> Client {
    pub fn auth(&'a mut self) -> ClientAuth<'a> {
        ClientAuth { client: self }
    }
}
