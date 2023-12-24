import { GetSecretOptions, InfisicalClient, LogLevel } from "./src";

// Just need a random string for testing, nothing crazy.
const randomStr = () => Date.now().toString(36);
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const client = new InfisicalClient({
    clientId: "92e6dae7-38ab-485d-8625-945a4f72c899",
    clientSecret: "082ca0e72bfb8391acb834a7471e52773fab90cb61a95d10764ade9327ef347e",
    siteUrl: "http://localhost:8080",
    cacheTtl: 10,
    logLevel: LogLevel.Debug
});

(async () => {
    const projectId = "6587ff06fe3abf0cb8bf1742";
    const environment = "dev";

    await client.listSecrets({ projectId, environment, attachToProcessEnv: true });

    console.log(process.env);

    /*
    while (true) {
        const promises: any = [];
        for (let i = 0; i < 1; i++) {
            promises.push(runTests());
        }

        await Promise.all(promises);
    }*/
})();

async function runTests() {
    const secretNames = ["TEST", "TEST2", "TEST3", "TEST4", "TEST5", "TEST6"];
    let promises: any = [];

    for (let i = 0; i++ < 10; i++) {
        const options: GetSecretOptions = {
            environment: "dev",
            projectId: "6587ff06fe3abf0cb8bf1742",

            secretName: secretNames[Math.floor(Math.random() * secretNames.length)]
        };

        promises.push(await client.getSecret(options));
    }

    await Promise.all(promises);
}
