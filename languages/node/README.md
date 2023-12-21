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

```js
import express from "express";

import { InfisicalClient, LogLevel } from "@infisical/sdk";

const app = express();

const PORT = 3000;

const client = new InfisicalClient({
    clientId: "YOUR_CLIENT_ID",
    clientSecret: "YOUR_CLIENT_SECRET",
    logLevel: LogLevel.Error
});

app.get("/", async (req, res) => {
    // access value

    const name = await client.getSecret({
        environment: "dev",
        projectId: "656dba7f979ebd6652586669",
        path: "/",
        type: "shared",
        secretName: "NAME"
    });

    res.send(`Hello! My name is: ${name.secretValue}`);
});

app.listen(PORT, async () => {
    // initialize client

    console.log(`App listening on port ${port}`);
});
```

# Installation

```console
$ npm install @infisical/sdk
```

# Configuration

Import the SDK and create a client instance with your [Machine Identity](http://infisical.com/docs/documentation/platform/identities/universal-auth).

```js
const { InfisicalClient, LogLevel } = require("@infisical/sdk");

const client = new InfisicalClient({
    clientId: "YOUR_CLIENT_ID",
    clientSecret: "YOUR_CLIENT_SECRET",
    logLevel: LogLevel.Error
});
```

Using ES6:

```js
import { InfisicalClient } from "@infisical/sdk";

const client = new InfisicalClient({
    clientId: "YOUR_CLIENT_ID",
    clientSecret: "YOUR_CLIENT_SECRET",
    logLevel: LogLevel.Error
});

// your app logic
```

### Options

We currently provide two ways to authenticate using Machine Identities. Either provide a direct access token that you can obtain from the [authentication API](https://infisical.com/docs/api-reference/endpoints/universal-auth/login), or provide your Machine Identity Client ID and client secret.

| Parameter | Type | Description |
| --- | --- | --- |
| `clientId` | `string` | Your machine identity client ID. |
| `clientSecret` | `string` | Your machine identity client secret. |
| `accessToken` | `string (optional)` | An access token obtained from the machine identity login endpoint. |
| `siteUrl` | `string (optional)` | Your self-hosted Infisical site URL. Default: `https://app.infisical.com`. |
| `logLevel` | `enum (optional)` | The level of logs you wish to log The logs are derived from Rust, as we have written our base SDK in Rust. Default: `Error`. |

# Secrets

## Get Secrets

```js
const secrets = await client.listSecrets({
    environment: "dev",
    projectId: "656dba7f979ebd6652586669",
    path: "/foo/bar/",
    includeImports: false
});
```

Retrieve all secrets within a given environment and folder path. The service token used must have access to the given path and environment.

### Options

| Parameter | Type | Description |
| --- | --- | --- |
| `environment` | `string` | The slug name (dev, prod, etc) of the environment from where secrets should be fetched from. |
| `projectId` | `string` | The project ID where the secret lives in. |
| `path` | `string (optional)` | The path from where secrets should be fetched from. |
| `includeImports` | `boolean, (optional)` | Whether or not to include imported secrets from the current path. Read about [secret import](https://infisical.com/docs/documentation/platform/secret-reference#import-entire-folders). |

## Get Secret

Retrieve a secret from Infisical:

```js
const secret = await client.getSecret({
    environment: "dev",
    projectId: "656dba7f979ebd6652586669",
    secretName: "API_KEY",
    path: "/",
    type: "shared"
});

const value = secret.secretValue; // get its value
```

By default, `getSecret()` fetches and returns a shared secret.

To explicitly retrieve a personal secret:

```js
const secret = await client.getSecret({
    environment: "dev",
    projectId: "656dba7f979ebd6652586669",
    secretName: "API_KEY",
    path: "/",
    type: "personal"
});

const value = secret.secretValue; // get its value
```

### Options

| Parameter | Type | Description |
| --- | --- | --- |
| `secretName` | `string` | The key of the secret to retrieve. |
| `projectId` | `string` | The project ID where the secret lives in. |
| `environment` | `string` | The slug name (dev, prod, etc) of the environment from where secrets should be fetched from. |
| `path` | `string (optional)` | The path from where secrets should be fetched from. |
| `type` | `string (optional)` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". |

## Create Secret

Create a new secret in Infisical:

```js
const newApiKey = await client.createSecret({
    projectId: "656dba7f979ebd6652586669",
    environment: "dev",
    secretName: "API_KEY",
    secretValue: "SECRET VALUE",
    path: "/",
    type: "shared"
});
```

### Options

| Parameter | Type | Description |
| --- | --- | --- |
| `secretName` | `string` | The key of the secret to create. |
| `secretValue` | `string` | The value of the secret. |
| `projectId` | `string` | The project ID where the secret lives in. |
| `environment` | `string` | The slug name (dev, prod, etc) of the environment where secret should be created |
| `path` | `string (optional)` | The path from where secret should be created. |
| `type` | `string, (optional)` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". A personal secret can only be created if a shared secret with the same name exists. |

## Update Secret

Update an existing secret in Infisical:

```js
const updatedApiKey = await client.updateSecret({
    secretName: "API_KEY",
    secretValue: "NEW SECRET VALUE",
    projectId: "656dba7f979ebd6652586669",
    environment: "dev",
    path: "/",
    type: "shared"
});
```

### Options

| Parameter | Type | Description |
| --- | --- | --- |
| `secretName` | `string` | The key of the secret to update. |
| `secretValue` | `string` | The new value of the secret. |
| `environment` | `string` | The slug name (dev, prod, etc) of the environment where secret should be updated. |
| `projectId` | `string` | The project ID where the secret lives in. |
| `path` | `string (optional)` | The path from where secret should be updated. |
| `type` | `string (optional)` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". |

## Delete Secret

Delete a secret in Infisical:

```js
const deletedSecret = await client.deleteSecret({
    secretName: "API_KEY",

    environment: "dev",
    projectId: "656dba7f979ebd6652586669",
    path: "/",

    type: "shared"
});
```

### Options

| Parameter | Type | Description |
| --- | --- | --- |
| `secretName` | `string` | The key of the secret to delete. |
| `projectId` | `string` | The project ID where the secret lives in. |
| `environment` | `string` | The slug name (dev, prod, etc) of the environment where secret should be deleted. |
| `path` | `string (optional)` | The path from where secret should be deleted. |
| `type` | `string, (optional)` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". Note that deleting a shared secret also deletes all associated personal secrets. |

# License

`Infisical Node.js SDK` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
