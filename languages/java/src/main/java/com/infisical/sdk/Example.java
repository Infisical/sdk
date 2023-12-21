package com.infisical.sdk;

import com.infisical.sdk.schema.*;

public class Example {

    public static void Main(String[] args) {
        ClientSettings settings = new ClientSettings();
        settings.setClientID("CLIENT_ID");
        settings.setClientSecret("CLIENT_SECRET");
        settings.setSiteURL("http://localhost:8080");

        InfisicalClient client = new InfisicalClient(settings);

        GetSecretOptions options = new GetSecretOptions();

        options.setSecretName("TEST");
        options.setEnvironment("dev");
        options.setProjectID("65670ae4f72abccd9f63d218");

        GetSecretResponseSecret secret = client.getSecret(options);

        System.out.println(secret.getSecretValue());

        client.close();
    }
}
