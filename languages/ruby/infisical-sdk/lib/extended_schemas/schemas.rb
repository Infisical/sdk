module InfisicalSDK
  # Command mapper
  class InfisicalCommands < Command
    attribute :get_secret,              Types.Constructor(GetSecretOptions).optional.default(nil)
    attribute :list_secrets,            Types.Constructor(ListSecretsOptions).optional.default(nil)
    attribute :create_secret,           Types.Constructor(CreateSecretOptions).optional.default(nil)
    attribute :update_secret,           Types.Constructor(UpdateSecretOptions).optional.default(nil)
    attribute :delete_secret,           Types.Constructor(DeleteSecretOptions).optional.default(nil)
    attribute :create_symmetric_key,    Types.Constructor(ArbitraryOptions).optional.default(nil)
    attribute :encrypt_symmetric,       Types.Constructor(EncryptSymmetricOptions).optional.default(nil)
    attribute :decrypt_symmetric,       Types.Constructor(DecryptSymmetricOptions).optional.default(nil)
    attribute :universal_auth_login,    Types.Constructor(UniversalAuthLoginClass).optional.default(nil)
    attribute :kubernetes_auth_login,   Types.Constructor(KubernetesAuthLoginClass).optional.default(nil)
    attribute :azure_auth_login,        Types.Constructor(AzureAuthLoginClass).optional.default(nil)
    attribute :gcp_id_token_auth_login, Types.Constructor(GcpIDTokenAuthLoginClass).optional.default(nil)
    attribute :gcp_iam_auth_login,      Types.Constructor(GcpIamAuthLoginClass).optional.default(nil)
    attribute :aws_iam_auth_login,      Types.Constructor(AwsIamAuthLoginClass).optional.default(nil)

    def to_dynamic
      {
        "getSecret"           => get_secret&.to_dynamic,
        "listSecrets"         => list_secrets&.to_dynamic,
        "createSecret"        => create_secret&.to_dynamic,
        "updateSecret"        => update_secret&.to_dynamic,
        "deleteSecret"        => delete_secret&.to_dynamic,
        "createSymmetricKey"  => create_symmetric_key&.to_dynamic,
        "encryptSymmetric"    => encrypt_symmetric&.to_dynamic,
        "decryptSymmetric"    => decrypt_symmetric&.to_dynamic,
        "universalAuthLogin"  => universal_auth_login&.to_dynamic,
        "kubernetesAuthLogin" => kubernetes_auth_login&.to_dynamic,
        "azureAuthLogin"      => azure_auth_login&.to_dynamic,
        "gcpIdTokenAuthLogin" => gcp_id_token_auth_login&.to_dynamic,
        "gcpIamAuthLogin"     => gcp_iam_auth_login&.to_dynamic,
        "awsIamAuthLogin"     => aws_iam_auth_login&.to_dynamic,
      }.compact
    end
  end
end
