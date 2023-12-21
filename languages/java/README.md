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
-   [Package](https://github.com/Infisical/sdk/packages/2019741)

# Basic Usage

```java
package com.example.app;

import com.infisical.sdk.InfisicalClient;
import com.infisical.sdk.schema.*;

public class Example {
    public static void main(String[] args) {
        // Create a new Infisical Client
        ClientSettings settings = new ClientSettings();
        settings.setClientID("MACHINE_IDENTITY_CLIENT_ID");
        settings.setClientSecret("MACHINE_IDENTITY_CLIENT_SECRET");

        InfisicalClient client = new InfisicalClient(settings);

        // Create the options for fetching the secret
        GetSecretOptions options = new GetSecretOptions();
        options.setSecretName("TEST");
        options.setEnvironment("dev");
        options.setProjectID("PROJECT_ID");

        // Fetch the sercret with the provided options
        GetSecretResponseSecret secret = client.getSecret(options);

        // Print the value
        System.out.println(secret.getSecretValue());

        // Important to avoid memory leaks! If you intend to reuse the client across
        // your entire application, you can omit this line.
        client.close();
    }
}
```

# Installation

The Infisical Java SDK is hosted on the GitHub Packages Apache Maven registry. Because of this you need to configure your environment properly so it's able to pull dependencies from the GitHub registry. Please check [this guide from GitHub](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-apache-maven-registry) on how to achieve this.

Our package is [located here](https://github.com/Infisical/sdk/packages/2019741). Please follow the installation guide on the page.

# Configuration

Import the SDK and create a client instance with your [Machine Identity](http://infisical.com/docs/documentation/platform/identities/universal-auth).

```java
import com.infisical.sdk.InfisicalClient;
import com.infisical.sdk.schema.*;
public class App {
    public static void main(String[] args) {

        ClientSettings settings = new ClientSettings();
        settings.setClientID("MACHINE_IDENTITY_CLIENT_ID");
        settings.setClientSecret("MACHINE_IDENTITY_CLIENT_SECRET");

        InfisicalClient client = new InfisicalClient(settings); // Your client!
    }
}

```

### ClientSettings options

We currently provide two ways to authenticate using Machine Identities. Either provide a direct access token that you can obtain from the [authentication API](https://infisical.com/docs/api-reference/endpoints/universal-auth/login), or provide your Machine Identity Client ID and client secret.

| Parameter      | Type                | Method              | Description                                                                |
| -------------- | ------------------- | ------------------- | -------------------------------------------------------------------------- |
| `clientID`     | `string`            | `setClientID()`     | Your machine identity client ID.                                           |
| `clientSecret` | `string`            | `setClientSecret()` | Your machine identity client secret.                                       |
| `accessToken`  | `string (optional)` | `setAccessToken()`  | An access token obtained from the machine identity login endpoint.         |
| `siteURL`      | `string (optional)` | `setSiteURL()`      | Your self-hosted Infisical site URL. Default: `https://app.infisical.com`. |

# Secrets

## Get Secrets

```java
ListSecretsOptions options = new ListSecretsOptions();
options.setEnvironment("dev");
options.setProjectID("656dba7f979ebd6652586669");
options.setPath("/foo/bar");
options.setIncludeImports(false);

SecretElement[] secrets = client.listSecrets(options);
```

Retrieve all secrets within a given environment and folder path. The service token used must have access to the given path and environment.

### Options

We currently provide two ways to authenticate using Machine Identities. Either provide a direct access token that you can obtain from the [authentication API](https://infisical.com/docs/api-reference/endpoints/universal-auth/login), or provide your Machine Identity Client ID and client secret.

| Parameter | Type | Method | Description |
| --- | --- | --- | --- |
| `environment` | `string` | `setEnvironment()` | The slug name (dev, prod, etc) of the environment from where secrets should be fetched from. |
| `projectID` | `string` | `setProjectID()` | The project ID where the secret lives in. |
| `path` | `string (optional)` | `setPath()` | The path from where secrets should be fetched from. |
| `includeImports` | `boolean, (optional)` | `setIncludeImports()` | Whether or not to include imported secrets from the current path. Read about [secret import](https://infisical.com/docs/documentation/platform/secret-reference#import-entire-folders). |

## Get Secret

Retrieve a secret from Infisical:

```java
GetSecretOptions options = new GetSecretOptions();
options.setSecretName("TEST");
options.setEnvironment("dev");
options.setProjectID("PROJECT_ID");

GetSecretResponseSecret secret = client.getSecret(options);

String secretValue = secret.getSecretValue();
```

By default, `getSecret()` fetches and returns a shared secret.

To explicitly retrieve a personal secret:

```java
 InfisicalClient client = new InfisicalClient(settings);

GetSecretOptions options = new GetSecretOptions();
options.setSecretName("TEST");
options.setEnvironment("dev");
options.setProjectID("PROJECT_ID");
options.setType("personal"); // <-- Add this!

GetSecretResponseSecret personalSecret = client.getSecret(options);

String secretValue = personalSecret.getSecretValue();
```

### Options

| Parameter | Type | Method | Description |
| --- | --- | --- | --- |
| `secretName` | `string` | `setSecretName()` | The key of the secret to retrieve. |
| `projectID` | `string` | `setProjectID()` | The project ID where the secret lives in. |
| `environment` | `string` | `setEnvironment()` | The slug name (dev, prod, etc) of the environment from where secrets should be fetched from. |
| `path` | `string (optional)` | `setPath()` | The path from where secrets should be fetched from. |
| `type` | `string (optional)` | `setType()` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". |

## Create Secret

Create a new secret in Infisical:

```java
CreateSecretOptions createOptions = new CreateSecretOptions();
createOptions.setSecretName("NEW_SECRET");
createOptions.setEnvironment("dev");
createOptions.setProjectID("PROJECT_ID");
createOptions.setSecretValue("SOME SECRET VALUE");
createOptions.setPath("/"); // Default
createOptions.setType("shared"); // Default

CreateSecretResponseSecret newSecret = client.createSecret(createOptions);
```

### Options

| Parameter | Type | Method | Description |
| --- | --- | --- | --- |
| `secretName` | `string` | `setSecretName()` | The key of the secret to create. |
| `secretValue` | `string` | `setSecretValue()` | The value of the secret. |
| `projectID` | `string` | `setProjectID()` | The project ID where the secret lives in. |
| `environment` | `string` | `setEnvironment()` | The slug name (dev, prod, etc) of the environment where secret should be created |
| `path` | `string (optional)` | `setPath()` | The path from where secret should be created. |
| `type` | `string, (optional)` | `setType()` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". A personal secret can only be created if a shared secret with the same name exists. |

## Update Secret

Update an existing secret in Infisical:

```java
UpdateSecretOptions options = new UpdateSecretOptions();

options.setSecretName("SECRET_TO_UPDATE");
options.setSecretValue("NEW SECRET VALUE");
options.setEnvironment("dev");
options.setProjectID("PROJECT_ID");
options.setPath("/"); // Default
options.setType("shared"); // Default

UpdateSecretResponseSecret updatedSecret = client.updateSecret(options);
```

### Options

| Parameter | Type | Method | Description |
| --- | --- | --- | --- |
| `secretName` | `string` | `setSecretName()` | The key of the secret to update. |
| `secretValue` | `string` | `setSecretValue()` | The new value of the secret. |
| `environment` | `string` | `setEnvironment()` | The slug name (dev, prod, etc) of the environment where secret should be updated. |
| `projectID` | `string` | `setProjectID()` | The project ID where the secret lives in. |
| `path` | `string (optional)` | `setPath()` | The path from where secret should be updated. |
| `type` | `string (optional)` | `setType()` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". |

## Delete Secret

Delete a secret in Infisical:

```java
DeleteSecretOptions options = new DeleteSecretOptions();

options.setSecretName("SECRET_TO_DELETE");
options.setEnvironment("dev");
options.setProjectID("PROJECT_ID");
options.setPath("/"); // Default
options.setType("shared"); // Default

DeleteSecretResponseSecret deletedSecret = client.deleteSecret(options);
```

### Options

| Parameter | Type | Method | Description |
| --- | --- | --- | --- |
| `secretName` | `string` | `setSecretName()` | The key of the secret to delete. |
| `projectID` | `string` | `setProjectID()` | The project ID where the secret lives in. |
| `environment` | `string` | `setEnvironment()` | The slug name (dev, prod, etc) of the environment where secret should be deleted. |
| `path` | `string (optional)` | `setPath()` | The path from where secret should be deleted. |
| `type` | `string, (optional)` | `setType()` | The type of the secret. Valid options are "shared" or "personal". If not specified, the default value is "shared". Note that deleting a shared secret also deletes all associated personal secrets. |

# License

`Infisical Java SDK` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
