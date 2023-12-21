package com.infisical.sdk;

import com.infisical.sdk.schema.*;

public class Example {

    public static void Main(String[] args) {
        ClientSettings settings = new ClientSettings();
        settings.setClientID("77719230-a0b6-4590-8fbd-376e8b0898a0");
        settings.setClientSecret("746d3b218d7841aa0a9e24d127f585f0d4d99aa7154de5a95918b0b774dc63ff");
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
