from infisical_client import InfisicalClient, GetSecretOptions, ClientSettings, DeleteSecretOptions, CreateSecretOptions, UpdateSecretOptions, ListSecretsOptions

client = InfisicalClient(ClientSettings(
    client_id="77719230-a0b6-4590-8fbd-376e8b0898a0",
    client_secret="4c9730a338dc64222114c473e8895311e5d34a1547e111fc173a67e418aed3a0",
    site_url="http://localhost:8080" # This is optional. If not provided, it will default to https://app.infisical.com
))

client.createSecret(options=CreateSecretOptions(
    secret_name="API_KEY",
    secret_value="Some API Key",
    environment="dev",
    project_id="658066938ffb84aa0aa507f6"
))

secret = client.getSecret(options=GetSecretOptions(
    environment="dev",
    project_id="658066938ffb84aa0aa507f6",
    secret_name="API_KEY",
    type="personal"
))


client.updateSecret(options=UpdateSecretOptions(
    secret_name="API_KEY",
    secret_value="new secret value!",
    environment="dev",
    project_id="658066938ffb84aa0aa507f6"
))


client.listSecrets(options=ListSecretsOptions(
    environment="dev",
    project_id="658066938ffb84aa0aa507f6",
))
client.deleteSecret(options=DeleteSecretOptions(
    environment="dev",
    project_id="658066938ffb84aa0aa507f6",
    secret_name="API_KEY"
))