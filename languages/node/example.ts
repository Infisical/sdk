import { GetSecretOptions, InfisicalClient, LogLevel } from "./src";

// Just need a random string for testing, nothing crazy.
const randomStr = () => Date.now().toString(36);
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const uncached_client = new InfisicalClient({
    clientId: "CLIENT_ID",
    clientSecret: "CLIENT_SECRET",
    siteUrl: "http://localhost:8080",
    cacheTtl: 0
});

const cached_client = new InfisicalClient({
    clientId: "CLIENT_ID",
    clientSecret: "CLIENT_SECRET",
    siteUrl: "http://localhost:8080",
    cacheTtl: 30
});

(async () => {
    /*const projectId = "6587ff06fe3abf0cb8bf1742";
    const environment = "dev";

    await uncached_client.getSecret({ projectId, environment, secretName: "TEST" });
    await cached_client.getSecret({ projectId, environment, secretName: "TEST" });
    
    console.time("uncached");
    for (let i = 0; i < 10; i++) {
        await uncached_client.getSecret({ projectId, environment, secretName: "TEST" }).then(console.log);
    }
    console.timeEnd("uncached");

    console.time("cached");
    for (let i = 0; i < 10; i++) {
        await cached_client.getSecret({ projectId, environment, secretName: "TEST" });
    }
    console.timeEnd("cached");

    process.exit(0);*/

    const getOptions = {
        projectId: "PROJECT_ID",
        environment: "dev",
        secretName: "TEST"
    } as const;

    const startSecret = await cached_client.getSecret(getOptions);

    console.time("fetch secret cached");
    await cached_client.getSecret(getOptions);
    console.timeEnd("fetch secret cached");

    // update the secret to remove from cache
    /*await cached_client.updateSecret({
        ...getOptions,
        secretValue: randomStr()
    });*/

    console.time("fetch secret uncached");
    await cached_client.getSecret(getOptions);
    console.timeEnd("fetch secret uncached");

    process.exit(0);
})();
