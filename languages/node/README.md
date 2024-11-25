If you're working with Node.js, the official [Infisical Node SDK](https://github.com/Infisical/sdk/tree/main/languages/node) package is the easiest way to fetch and work with secrets for your application.

- [NPM Package](https://www.npmjs.com/package/@infisical/sdk)
- [Github Repository](https://github.com/Infisical/sdk/tree/main/languages/node)

## Basic Usage

```js
import express from "express";

import { InfisicalClient } from "@infisical/sdk";

const app = express();

const PORT = 3000;

const client = new InfisicalClient({
    siteUrl: "https://app.infisical.com", // Optional, defaults to https://app.infisical.com
    auth: {
      universalAuth: {
        clientId: "YOUR_CLIENT_ID",
        clientSecret: "YOUR_CLIENT_SECRET"
      }
    }
});

app.get("/", async (req, res) => {
  // Access the secret
  
    const name = await client.getSecret({
        environment: "dev",
        projectId: "PROJECT_ID",
        path: "/",
        type: "shared",
        secretName: "NAME"
    });

    res.send(`Hello! My name is: ${name.secretValue}`);
});

app.listen(PORT, async () => {
    // initialize client

    console.log(`App listening on port ${PORT}`);
});
```

This example demonstrates how to use the Infisical Node SDK with an Express application. The application retrieves a secret named "NAME" and responds to requests with a greeting that includes the secret value.

<Warning>
    We do not recommend hardcoding your [Machine Identity Tokens](/documentation/platform/identities/overview). Setting it as an environment variable
    would be best.
</Warning>

## Installation

Run `npm` to add `@infisical/sdk` to your project.

```console
$ npm install @infisical/sdk
```

## Configuration

Import the SDK and create a client instance with your [Machine Identity](/documentation/platform/identities/overview).

<Tabs>
  <Tab title="ES6">
    ```js
    import { InfisicalClient, LogLevel } from "@infisical/sdk";

    const client = new InfisicalClient({
        auth: {
          universalAuth: {
            clientId: "YOUR_CLIENT_ID",
            clientSecret: "YOUR_CLIENT_SECRET"
          }
        },
        logLevel: LogLevel.Error
    });
    ```

  </Tab>
  <Tab title="ES5">
    ```js
    const { InfisicalClient } = require("@infisical/sdk");

    const client = new InfisicalClient({
        auth: {
          universalAuth: {
            clientId: "YOUR_CLIENT_ID",
            clientSecret: "YOUR_CLIENT_SECRET"
          }
        },
    });
    ```

  </Tab>
</Tabs>

## Parameters

### `clientId` (deprecated)
- **Type**: `string`
- **Optional**: Yes
- **Description**: Your machine identity client ID.

> **Deprecation Notice**: This field is deprecated and will be removed in future versions. Please use the `auth.universalAuth.clientId` field instead.

### `clientSecret` (deprecated)
- **Type**: `string`
- **Optional**: Yes
- **Description**: Your machine identity client secret.

> **Deprecation Notice**: This field is deprecated and will be removed in future versions. Please use the `auth.universalAuth.clientSecret` field instead.

### `accessToken` (deprecated)
- **Type**: `string`
- **Optional**: Yes
- **Description**: An access token obtained from the machine identity login endpoint.

> **Deprecation Notice**: This field is deprecated and will be removed in future versions. Please use the `auth.accessToken` field instead.

### `cacheTtl`
- **Type**: `number`
- **Default**: `300`
- **Optional**: Yes
- **Description**: Time-to-live (in seconds) for refreshing cached secrets. If manually set to 0, caching will be disabled, this is not recommended.

### `siteUrl`
- **Type**: `string`
- **Default**: `"https://app.infisical.com"`
- **Optional**: Yes
- **Description**: Your self-hosted absolute site URL including the protocol (e.g. `https://app.infisical.com`)

### `logLevel`
- **Type**: `enum`
- **Default**: `"Error"`
- **Optional**: Yes
- **Description**: The level of logs you wish to log. The logs are derived from Rust, as we have written our base SDK in Rust.

### `sslCertificatePath`
- **Type**: `string`
- **Optional**: Yes
- **Description**: Optionally provide a path to a custom SSL certificate file. This can be substituted by setting the `INFISICAL_SSL_CERTIFICATE` environment variable to the contents of the certificate.

### `auth`
- **Type**: `AuthenticationOptions`
- **Optional**: No (required unless using environment variables)
- **Description**: The authentication object to use for the client. This is required unless you're using environment variables.


### Authentication

The SDK supports a variety of authentication methods. The most common authentication method is Universal Auth, which uses a client ID and client secret to authenticate.

#### Universal Auth

**Using environment variables**
- `INFISICAL_UNIVERSAL_AUTH_CLIENT_ID` - Your machine identity client ID.
- `INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET` - Your machine identity client secret.

**Using the SDK directly**
```js
const client = new InfisicalClient({
    auth: {
      universalAuth: {
        clientId: "YOUR_CLIENT_ID",
        clientSecret: "YOUR_CLIENT_SECRET"
      }
    }
});
```

#### GCP ID Token Auth
<Info>
  Please note that this authentication method will only work if you're running your application on Google Cloud Platform.
  Please [read more](/documentation/platform/identities/gcp-auth) about this authentication method.
</Info>

**Using environment variables**
- `INFISICAL_GCP_AUTH_IDENTITY_ID` - Your Infisical Machine Identity ID.

**Using the SDK directly**
```js
const client = new InfisicalClient({
    auth: {
      gcpIdToken: {
        identityId: "YOUR_IDENTITY_ID"
      }
    }
});
```

#### GCP IAM Auth

**Using environment variables**
- `INFISICAL_GCP_IAM_AUTH_IDENTITY_ID` - Your Infisical Machine Identity ID.
- `INFISICAL_GCP_IAM_SERVICE_ACCOUNT_KEY_FILE_PATH` - The path to your GCP service account key file.

**Using the SDK directly**
```js
const client = new InfisicalClient({
    auth: {
      gcpIam: {
        identityId: "YOUR_IDENTITY_ID",
        serviceAccountKeyFilePath: "./path/to/your/service-account-key.json"
      }
    }
});
```

#### AWS IAM Auth
_Please note that this authentication method will only work if you're running your application on AWS.
Please [read more](/documentation/platform/identities/aws-auth) about this authentication method._


**Using environment variables**
- `INFISICAL_AWS_IAM_AUTH_IDENTITY_ID` - Your Infisical Machine Identity ID.

**Using the SDK directly**
```js
const client = new InfisicalClient({
    auth: {
      awsIam: {
        identityId: "YOUR_IDENTITY_ID"
      }
    }
});
```

#### Azure Auth
_Please note that this authentication method will only work if you're running your application on Azure.
Please [read more](/documentation/platform/identities/azure-auth) about this authentication method._

**Using environment variables**
- `INFISICAL_AZURE_AUTH_IDENTITY_ID` - Your Infisical Machine Identity ID.

**Using the SDK directly**
```js
const client = new InfisicalClient({
  auth: {
    azure: {
      identityId: "YOUR_IDENTITY_ID"
    }
  }
});
```


#### Kubernetes Auth
<Info>
  Please note that this authentication method will only work if you're running your application on Kubernetes.
  Please [read more](/documentation/platform/identities/kubernetes-auth) about this authentication method.
</Info>

**Using environment variables**
- `INFISICAL_KUBERNETES_IDENTITY_ID` - Your Infisical Machine Identity ID.
- `INFISICAL_KUBERNETES_SERVICE_ACCOUNT_TOKEN_PATH_ENV_NAME` - The environment variable name that contains the path to the service account token. This is optional and will default to `/var/run/secrets/kubernetes.io/serviceaccount/token`.

**Using the SDK directly**
```js
const client = new InfisicalClient({
  auth: {
    kubernetes: {
      identityId: "YOUR_IDENTITY_ID",
      serviceAccountTokenPathEnvName: "/var/run/secrets/kubernetes.io/serviceaccount/token" // Optional
    }
  }
});
```

### Caching

To reduce the number of API requests, the SDK temporarily stores secrets it retrieves. By default, a secret remains cached for 5 minutes after it's first fetched. Each time it's fetched again, this 5-minute timer resets. You can adjust this caching duration by setting the "cacheTtl" option when creating the client.

## Working with Secrets

### client.listSecrets(options)

```js
const secrets = await client.listSecrets({
    environment: "dev",
    projectId: "PROJECT_ID",
    path: "/foo/bar/",
    includeImports: false
});
```

Retrieve all secrets within the Infisical project and environment that client is connected to

#### Parameters

##### `environment`
- **Type**: `string`
- **Required**: Yes
- **Description**: The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.

##### `projectId`
- **Type**: `string`
- **Required**: Yes
- **Description**: The project ID where the secret lives in.

##### `path`
- **Type**: `string`
- **Optional**: Yes
- **Description**: The path from where secrets should be fetched from.

##### `attachToProcessEnv`
- **Type**: `boolean`
- **Default**: `false`
- **Optional**: Yes
- **Description**: Whether or not to set the fetched secrets to the process environment. If true, you can access the secrets like so `process.env["SECRET_NAME"]`.

##### `recursive`
- **Type**: `boolean`
- **Default**: `false`
- **Optional**: Yes
- **Description**: Whether or not to fetch secrets recursively from the specified path. Please note that there's a 20-depth limit for recursive fetching.

##### `expandSecretReferences`
- **Type**: `boolean`
- **Default**: `true`
- **Optional**: Yes
- **Description**: Whether or not to expand secret references in the fetched secrets. Read about [secret reference](/documentation/platform/secret-reference)

##### `includeImports`
- **Type**: `boolean`
- **Default**: `false`
- **Optional**: Yes
- **Description**: Whether or not to include imported secrets from the current path. Read about [secret import](/documentation/platform/secret-reference)

### client.getSecret(options)

```js
const secret = await client.getSecret({
    environment: "dev",
    projectId: "PROJECT_ID",
    secretName: "API_KEY",
    path: "/",
    type: "shared"
});
```

Retrieve a secret from Infisical.

By default, `getSecret()` fetches and returns a shared secret.

#### Parameters

##### `secretName`
- **Type**: `string`
- **Required**: Yes
- **Description**: The key of the secret to retrieve.

##### `projectId`
- **Type**: `string`
- **Required**: Yes
- **Description**: The project ID where the secret lives in.

##### `environment`
- **Type**: `string`
- **Required**: Yes
- **Description**: The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.

##### `path`
- **Type**: `string`
- **Optional**: Yes
- **Description**: The path from where secret should be fetched from.

##### `type`
- **Type**: `string`
- **Optional**: Yes
- **Default**: `"shared"`
- **Valid Options**: `"shared"` or `"personal"`
- **Description**: The type of the secret.

##### `includeImports`
- **Type**: `boolean`
- **Optional**: Yes
- **Description**: Whether or not to include imported secrets from the current path. Read about [secret import](/documentation/platform/secret-reference)

##### `expandSecretReferences`
- **Type**: `boolean`
- **Default**: `true`
- **Optional**: Yes
- **Description**: Whether or not to expand secret references in the fetched secrets. Read about [secret reference](/documentation/platform/secret-reference)

### client.createSecret(options)

```js
const newApiKey = await client.createSecret({
    projectId: "PROJECT_ID",
    environment: "dev",
    secretName: "API_KEY",
    secretValue: "SECRET VALUE",
    path: "/",
    type: "shared"
});
```

Create a new secret in Infisical.

#### Parameters

##### `secretName`
- **Type**: `string`
- **Required**: Yes
- **Description**: The key of the secret to create.

##### `secretValue`
- **Type**: `string`
- **Required**: Yes
- **Description**: The value of the secret.

##### `projectId`
- **Type**: `string`
- **Required**: Yes
- **Description**: The project ID where the secret lives in.

##### `environment`
- **Type**: `string`
- **Required**: Yes
- **Description**: The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.

##### `path`
- **Type**: `string`
- **Optional**: Yes
- **Description**: The path from where secret should be created.

##### `type`
- **Type**: `string`
- **Optional**: Yes
- **Default**: `"shared"`
- **Valid Options**: `"shared"` or `"personal"`
- **Description**: The type of the secret.

### client.updateSecret(options)

```js
const updatedApiKey = await client.updateSecret({
    secretName: "API_KEY",
    secretValue: "NEW SECRET VALUE",
    projectId: "PROJECT_ID",
    environment: "dev",
    path: "/",
    type: "shared"
});
```

Update an existing secret in Infisical.

#### Parameters

##### `secretName`
- **Type**: `string`
- **Required**: Yes
- **Description**: The key of the secret to update.

##### `secretValue`
- **Type**: `string`
- **Required**: Yes
- **Description**: The new value of the secret.

##### `projectId`
- **Type**: `string`
- **Required**: Yes
- **Description**: The project ID where the secret lives in.

##### `environment`
- **Type**: `string`
- **Required**: Yes
- **Description**: The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.

##### `path`
- **Type**: `string`
- **Optional**: Yes
- **Description**: The path from where secret should be updated.

##### `type`
- **Type**: `string`
- **Optional**: Yes
- **Default**: `"shared"`
- **Valid Options**: `"shared"` or `"personal"`
- **Description**: The type of the secret.

##### client.deleteSecret(options)

```js
const deletedSecret = await client.deleteSecret({
    secretName: "API_KEY",

    environment: "dev",
    projectId: "PROJECT_ID",
    path: "/",

    type: "shared"
});
```

Delete a secret in Infisical.

#### Parameters

##### `secretName`
- **Type**: `string`
- **Required**: Yes
- **Description**: The key of the secret to update.

##### `projectId`
- **Type**: `string`
- **Required**: Yes
- **Description**: The project ID where the secret lives in.

##### `environment`
- **Type**: `string`
- **Required**: Yes
- **Description**: The slug name (dev, prod, etc) of the environment from where secrets should be fetched from.

##### `path`
- **Type**: `string`
- **Optional**: Yes
- **Description**: The path from where secret should be deleted.

##### `type`
- **Type**: `string`
- **Optional**: Yes
- **Default**: `"shared"`
- **Valid Options**: `"shared"` or `"personal"`
- **Description**: The type of the secret.

## Cryptography

### Create a symmetric key

Create a base64-encoded, 256-bit symmetric key to be used for encryption/decryption.

```js
const key = client.createSymmetricKey();
```

#### Returns (string)
`key` (string): A base64-encoded, 256-bit symmetric key, that can be used for encryption/decryption purposes.

### Encrypt symmetric
```js
const { iv, tag, ciphertext } = await client.encryptSymmetric({
    key: key,
    plaintext: "Infisical is awesome!",
})
```

#### Parameters

##### `plaintext`
- **Type**: `string`
- **Required**: Yes
- **Description**: The plaintext you want to encrypt.

##### `key`
- **Type**: `string`
- **Required**: Yes
- **Description**: The symmetric key to use for encryption.

#### Returns (object)
`tag` (string): A base64-encoded, 128-bit authentication tag.
`iv` (string): A base64-encoded, 96-bit initialization vector.
`ciphertext` (string): A base64-encoded, encrypted ciphertext.

### Decrypt symmetric
```js
const decryptedString = await client.decryptSymmetric({
    key: key,
    iv: iv,
    tag: tag,
    ciphertext: ciphertext,
});
```

#### Parameters

##### `ciphertext`
- **Type**: `string`
- **Required**: Yes
- **Description**: The ciphertext you want to decrypt.

##### `key`
- **Type**: `string`
- **Required**: Yes
- **Description**: The symmetric key to use for encryption.

##### `iv`
- **Type**: `string`
- **Required**: Yes
- **Description**: The initialization vector to use for decryption.

##### `tag`
- **Type**: `string`
- **Required**: Yes
- **Description**: The authentication tag to use for decryption.

#### Returns (string)
`plaintext` (string): The decrypted plaintext.