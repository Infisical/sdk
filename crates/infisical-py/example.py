from infisical_client import InfisicalClient, GetSecretOptions, ClientSettings, DeleteSecretOptions, CreateSecretOptions, UpdateSecretOptions, ListSecretsOptions
import time
client = InfisicalClient(ClientSettings(
    client_id="92e6dae7-38ab-485d-8625-945a4f72c899",
    client_secret="082ca0e72bfb8391acb834a7471e52773fab90cb61a95d10764ade9327ef347e",
    site_url="http://localhost:8080", # This is optional. If not provided, it will default to https://app.infisical.com
    cache_ttl=5,
))

client.getSecret(options=GetSecretOptions(
    environment="dev",
    project_id="6587ff06fe3abf0cb8bf1742",
    secret_name="TEST"
))

# Test rust multi threaded cache (should be super fast)
while True:
    secret = client.getSecret(options=GetSecretOptions(
        environment="dev",
        project_id="6587ff06fe3abf0cb8bf1742",
        secret_name="TEST"
    ))
    print(secret.secret_value)