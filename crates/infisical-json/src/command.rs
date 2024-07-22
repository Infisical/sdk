use infisical::manager::secrets::{
    CreateSecretOptions, DeleteSecretOptions, GetSecretOptions, ListSecretsOptions,
    UpdateSecretOptions,
};

use infisical::manager::cryptography::{DecryptSymmetricOptions, EncryptSymmetricOptions};

use infisical::client::auth_method_settings::{
    AWSIamAuthMethod, AzureAuthMethod, GCPIamAuthMethod, GCPIdTokenAuthMethod,
    KubernetesAuthMethod, UniversalAuthMethod,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
// QuickType (our type generator), won't recognize the CreateSymmetricKey command unless it has an input. Super annoying, and this is quite a hacky workaround.
// This should be revised in the future.
pub struct ArbitraryOptions {
    pub data: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Command {
    GetSecret(GetSecretOptions),
    ListSecrets(ListSecretsOptions),
    CreateSecret(CreateSecretOptions),
    UpdateSecret(UpdateSecretOptions),
    DeleteSecret(DeleteSecretOptions),

    CreateSymmetricKey(ArbitraryOptions),
    EncryptSymmetric(EncryptSymmetricOptions),
    DecryptSymmetric(DecryptSymmetricOptions),

    UniversalAuthLogin(UniversalAuthMethod),
    KubernetesAuthLogin(KubernetesAuthMethod),
    AzureAuthLogin(AzureAuthMethod),
    GcpIdTokenAuthLogin(GCPIdTokenAuthMethod),
    GcpIamAuthLogin(GCPIamAuthMethod),
    AwsIamAuthLogin(AWSIamAuthMethod),
}
