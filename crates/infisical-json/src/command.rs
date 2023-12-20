use infisical::manager::{
    client_secrets::{CreateSecretOptions, GetSecretOptions, UpdateSecretOptions},
    secrets::{DeleteSecretOptions, ListSecretsOptions},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]

// We expand this later
pub enum Command {
    GetSecret(GetSecretOptions),
    ListSecrets(ListSecretsOptions),
    CreateSecret(CreateSecretOptions),
    UpdateSecret(UpdateSecretOptions),
    DeleteSecret(DeleteSecretOptions),
}
