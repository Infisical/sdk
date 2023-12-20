import { InfisicalClient, LogLevel } from "./src";

// Just need a random string for testing, nothing crazy.
const randomStr = () => Date.now().toString(36);

(async () => {
    const client = new InfisicalClient({
        clientId: "77719230-a0b6-4590-8fbd-376e8b0898a0",
        clientSecret: "4c9730a338dc64222114c473e8895311e5d34a1547e111fc173a67e418aed3a0",
        siteUrl: "http://localhost:8080",
        logLevel: LogLevel.Debug
    });

    const newSecretName = "123";
    const projectId = "658066938ffb84aa0aa507f6";
    const environment = "dev";

    const newSecret = await client.createSecret({
        environment,
        projectId,

        secretName: newSecretName,
        secretValue: "test"
    });

    console.log("Created secret:", newSecret);
    console.log("\n\n\n\n\n\n\n\n\n");

    const secret = await client.getSecret({
        environment,
        projectId,

        secretName: newSecretName
    });

    console.log("Got secret:", secret);
    console.log("\n\n\n\n\n\n\n\n\n");

    const secrets = await client.listSecrets({
        environment,
        projectId
    });

    console.log("Listed secrets:", secrets);
    console.log("\n\n\n\n\n\n\n\n\n");

    const updatedSecret = await client.updateSecret({
        environment,
        projectId,

        secretName: newSecretName,
        secretValue: "NEW VALUE"
    });

    console.log("Updated secret:", updatedSecret);
    console.log("\n\n\n\n\n\n\n\n\n");

    /* const deletedSecret = await client.deleteSecret({
        environment,
        projectId,

        secretName: secretName
    });*/

    //console.log("Deleted secret:", deletedSecret);
    console.log("\n\n\n\n\n\n\n\n\n");

    const secretsAfterDelete = await client.listSecrets({
        environment,
        projectId
    });

    console.log("Listed secrets after delete:", secretsAfterDelete);
    console.log("\n\n\n\n\n\n\n\n\n");
})();
