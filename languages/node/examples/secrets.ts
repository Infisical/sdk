import { InfisicalClient, LogLevel } from "../src";

// Just need a random string for testing, nothing crazy.
const randomStr = () => Date.now().toString(36);
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const uncachedClient = new InfisicalClient({
    clientId: "CLIENT_ID",
    clientSecret: "CLIENT_SECRET",
    siteUrl: "http://localhost:8080", // This is optional. You can remove it and it will default to https://app.infisical.com.
    cacheTtl: 0, // Default is 300 seconds (5 minutes).
    logLevel: LogLevel.Error // Optional, default is LogLevel.Error.
});

const cachedClient = new InfisicalClient({
    clientId: "CLIENT_ID",
    clientSecret: "CLIENT_SECRET",
    siteUrl: "http://localhost:8080", // This is optional. You can remove it and it will default to https://app.infisical.com.
    cacheTtl: 300, // Default is 300 seconds (5 minutes).
    logLevel: LogLevel.Error // Optional, default is LogLevel.Error.
});

// Make sure to change these values.
const projectId = "YOUR_PROJECT_ID";
const environment = "dev";

async function main() {
    await testCacheSpeeds();

    // Get a secret, and update it afterwards, as an example.
    const secretOptions = {
        projectId: projectId,
        environment: environment,
        secretName: "TEST"
    } as const;

    const secret = await cachedClient.getSecret(secretOptions);
    console.log("Fetched secret", secret);

    const updatedSecret = await cachedClient.updateSecret({
        ...secretOptions,
        secretValue: "NEW VALUE"
    });
    console.log("Updated secret", updatedSecret);
}

async function testCacheSpeeds() {
    console.log("Testing cache speeds...");

    await uncachedClient.getSecret({ projectId, environment, secretName: "TEST" });
    await cachedClient.getSecret({ projectId, environment, secretName: "TEST" });

    const startUncached = Date.now();
    for (let i = 0; i < 10; i++) await uncachedClient.getSecret({ projectId, environment, secretName: "TEST" }).then(console.log);
    const endUncached = Date.now();

    const startCached = Date.now();
    for (let i = 0; i < 10; i++) await cachedClient.getSecret({ projectId, environment, secretName: "TEST" });
    const endCached = Date.now();

    console.log(`Uncached: ${endUncached - startUncached}ms`);
    console.log(`Cached: ${endCached - startCached}ms\n`);

    const percentage = (endUncached - startUncached) / (endCached - startCached);
    console.log(`Cached fetched the same secret 10 times, ${percentage.toFixed(2)}x faster than uncached`);
}

main();
