<h1 align="center">
    <a href="https://github.com/Infisical/infisical">
        <img width="300" src="https://raw.githubusercontent.com/Infisical/infisical-node/main/img/logoname-white.svg#gh-dark-mode-only" alt="infisical">
    </a>
</h1>
<p align="center">
  <p align="center">Open-source, end-to-end encrypted tool to manage secrets and configs across your team and infrastructure.</p>
</p>

# Table of Contents

-   [Links](#links)
-   [Basic Usage](#basic-usage)
-   [Secrets](#working-with-secrets)
    -   [Get Secrets](#get-secrets)
    -   [Get Secret](#get-secret)
    -   [Create Secret](#create-secret)
    -   [Update Secret](#update-secret)
    -   [Delete Secret](#delete-secret)

# Links

-   [Infisical](https://github.com/Infisical/infisical)

# Basic Usage

```py
from flask import Flask
from infisical_client import ClientSettings, InfisicalClient, GetSecretOptions

app = Flask(__name__)

client = InfisicalClient(ClientSettings(
    client_id="MACHINE_IDENTITY_CLIENT_ID",
    client_secret="MACHINE_IDENTITY_CLIENT_SECRET",
))

@app.route("/")
def hello_world():
    # access value

    name = client.getSecret(options=GetSecretOptions(
       environment="dev",
       project_id="PROJECT_ID",
       secret_name="NAME"
    ))

    return f"Hello! My name is: {name.secret_value}"
```

This example demonstrates how to use the Infisical Python SDK with a Flask application. The application retrieves a secret named "NAME" and responds to requests with a greeting that includes the secret value.

# Installation

You need Python 3.7+.

```console
$ pip install infisical-python
```

# Configuration

Import the SDK and create a client instance with your [Machine Identity](https://infisical.com/docs/api-reference/overview/authentication).

```py
from infisical_client import ClientSettings, InfisicalClient

client = InfisicalClient(ClientSettings(
    client_id="MACHINE_IDENTITY_CLIENT_ID",
    client_secret="MACHINE_IDENTITY_CLIENT_SECRET",
))
```

### Options

| Parameter       | Type     | Description                                                                                             |
| --------------- | -------- | ------------------------------------------------------------------------------------------------------- |
| `client_id`     | `string` | Your Infisical Client ID.                                                                               |
| `client_secret` | `string` | Your Infisical Client Secret.                                                                           |
| `access_token`  | `string` | If you want to directly pass an access token obtained from the authentication endpoints, you can do so. |
| `site_url`      | `string` | Your self-hosted Infisical site URL. Default: `https://app.infisical.com`.                              |

# Secrets

## List secrets

```py
client.listSecrets(options=ListSecretsOptions(
    environment="dev",
    project_id="658066938ffb84aa0aa507f6"
))
```

Retrieve all secrets within a given environment and folder path. The service token used must have access to the given path and environment.

### Parameters

-   `environment` (string): The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.
-   `project_id` (string): The ID of the project the secret lives in.
-   `path` (string): The path from where secrets should be fetched from.
-   `include_imports` (boolean): Whether or not to include imported secrets from the current path. Read about [secret import](https://infisical.com/docs/documentation/platform/secret-reference#import-entire-folders). If not specified, the default value is `True`.

## Get Secret

```py
secret = client.getSecret(options=GetSecretOptions(
    environment="dev",
    project_id="658066938ffb84aa0aa507f6",
    secret_name="API_KEY"
))
value = secret.secret_value # get its value
```

By default, `get_secret()` fetches and returns a shared secret. If not found, it returns a personal secret.

To explicitly retrieve a personal secret:

```py
secret = client.getSecret(options=GetSecretOptions(
    environment="dev",
    project_id="658066938ffb84aa0aa507f6",
    secret_name="API_KEY",
    type="personal"
))
value = secret.secret_value # get its value
```

### Parameters

-   `secret_name` (string): The key of the secret to retrieve.
-   `environment` (string): The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.
-   `project_id` (string): The ID of the project the secret lives in.
-   `path` (string): The path from where secrets should be fetched from.
-   `type` (string, optional): The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "personal".
-   `include_imports` (boolean, optional): Whether or not to include imported secrets from the current path. Read about secret import.

## Create Secret

Create a new secret in Infisical

```py
api_key = client.createSecret(options=CreateSecretOptions(
    secret_name="API_KEY",
    secret_value="Some API Key",
    environment="dev",
    project_id="658066938ffb84aa0aa507f6"
))
```

### Parameters

-   `secret_name` (string): The key of the secret to create.
-   `secret_value` (string): The value of the secret.
-   `environment` (string): The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.
-   `project_id` (string): The ID of the project the secret lives in.
-   `path` (string): The path from where secrets should be created.
-   `type` (string, optional): The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". A personal secret can only be created if a shared secret with the same name exists.

## Update Secret

Update an existing secret in Infisical

```py
client.updateSecret(options=UpdateSecretOptions(
    secret_name="API_KEY",
    secret_value="new secret value!",
    environment="dev",
    project_id="658066938ffb84aa0aa507f6"
))
```

### Parameters

-   `secret_name` (string): The key of the secret to update.
-   `secret_value` (string): The new value of the secret.
-   `environment` (string): The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.
-   `project_id` (string): The ID of the project the secret lives in.
-   `path` (string): The path from where secrets should be updated.
-   `type` (string, optional): The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared".

## Delete Secret

Delete a secret in Infisical

```py
client.deleteSecret(options=DeleteSecretOptions(
    environment="dev",
    project_id="658066938ffb84aa0aa507f6",
    secret_name="API_KEY"
))
```

### Parameters

-   `secret_name` (string): The key of the secret to delete.
-   `environment` (string): The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.
-   `project_id` (string): The ID of the project the secret lives in.
-   `path` (string): The path from where secrets should be deleted.
-   `type` (string, optional): The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared".

# License

`infisical-python` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
