import { InfisicalClient, ListSecretsOptions, LogLevel } from "../src";

// Just need a random string for testing, nothing crazy.
const randomStr = () => Date.now().toString(36);

// Make sure to change these values.
const projectId = "PROJECT_ID";
const environment = "dev";
const TEST_SECRET_NAME = "DATABASE_URL";

const MACHINE_IDENTITY_CLIENT_SECRET = "CLIENT_SECRET";
const MACHINE_IDENTITY_CLIENT_ID = "CLIENT_ID";

const uncachedClient = new InfisicalClient({
	clientId: MACHINE_IDENTITY_CLIENT_ID,
	clientSecret: MACHINE_IDENTITY_CLIENT_SECRET,
	// siteUrl: "http://localhost:8080", // This is optional. You can remove it and it will default to https://app.infisical.com.
	cacheTtl: 0 // Default is 300 seconds (5 minutes). (0 means no caching)
	// logLevel: LogLevel.Debug // Optional, default is LogLevel.Error.
});

const cachedClient = new InfisicalClient({
	clientId: MACHINE_IDENTITY_CLIENT_ID,
	clientSecret: MACHINE_IDENTITY_CLIENT_SECRET,
	// siteUrl: "http://localhost:8080", // This is optional. You can remove it and it will default to https://app.infisical.com.
	cacheTtl: 300, // Default is 300 seconds (5 minutes).
	logLevel: LogLevel.Error // Optional, default is LogLevel.Error.
});

async function main() {
	await testCacheSpeeds();
	await testListSecrets();
	await testCreateSecret();
	await testDeleteSecret();
	await testUpdateSecret();
}

async function testUpdateSecret() {
	console.log("⏱️ Updating secret");
	const newValue = randomStr();

	const updatedSecret = await uncachedClient.updateSecret({
		secretName: TEST_SECRET_NAME,
		environment,
		projectId,
		secretValue: newValue
	});

	if (updatedSecret.secretValue !== newValue) {
		throw new Error("❌ testUpdate: secretValue was not the same");
	}

	console.log("✅ Secret updated");
}

async function testDeleteSecret() {
	console.log("⏱️ Deleting secret");

	const secretName = randomStr();
	const secretValue = randomStr();
	await uncachedClient.createSecret({
		secretName,
		secretValue,
		projectId,
		environment
	});

	const deletedSecret = await uncachedClient.deleteSecret({
		secretName,
		environment,
		projectId
	});

	if (deletedSecret.secretKey !== secretName) {
		throw new Error("❌ testDelete: secretName was not the same");
	}

	console.log("✅ Secret deleted");
}

async function testCreateSecret() {
	console.log("⏱️ Creating secret");

	const secretName = randomStr();
	const secretValue = randomStr();
	const createdSecret = await uncachedClient.createSecret({
		secretName,
		secretValue,
		projectId,
		environment
	});

	if (createdSecret.secretKey !== secretName) {
		throw new Error("❌ testCreate: secretName was not the same");
	}

	await uncachedClient.deleteSecret({
		secretName,
		environment,
		projectId
	});

	console.log("✅ Secret created");
}

async function testListSecrets() {
	console.log("⏱️ Listing secrets");

	const secretName = randomStr();
	const secretValue = randomStr();
	await uncachedClient.createSecret({
		secretName,
		secretValue,
		projectId,
		environment
	});

	const secrets = await uncachedClient.listSecrets({
		projectId,
		recursive: true, // Optional, default is false
		expandSecretReferences: true, // Optional, default is true. This will expand secret references in the response.
		environment
	});

	secrets.forEach(sec => console.log(`Secret: ${sec.secretKey}   ---   ${sec.secretValue}`));
	console.log(`Total secrets: ${secrets.length}`);

	if (!secrets.length) {
		throw new Error("❌ testList: secrets.length was 0");
	}

	await uncachedClient.deleteSecret({
		secretName,
		environment,
		projectId
	});

	console.log("✅ Secrets listed");
}

async function testCacheSpeeds() {
	console.log("Testing cache speeds...");

	await uncachedClient.getSecret({ projectId, environment, secretName: TEST_SECRET_NAME });
	await cachedClient.getSecret({ projectId, environment, secretName: TEST_SECRET_NAME });

	const startUncached = Date.now();
	for (let i = 0; i < 10; i++) await uncachedClient.getSecret({ projectId, environment, secretName: TEST_SECRET_NAME });
	const endUncached = Date.now();

	const startCached = Date.now();
	for (let i = 0; i < 10; i++) await cachedClient.getSecret({ projectId, environment, secretName: TEST_SECRET_NAME });
	const endCached = Date.now();

	console.log(`Uncached: ${endUncached - startUncached}ms`);
	console.log(`Cached: ${endCached - startCached}ms\n`);

	const percentage = (endUncached - startUncached) / (endCached - startCached);
	console.log(`Cached fetched the same secret 10 times, ${percentage.toFixed(2)}x faster than uncached\n\n`);
}

main();
