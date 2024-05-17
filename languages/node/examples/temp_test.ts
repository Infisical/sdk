import { InfisicalClient, ListSecretsOptions, LogLevel } from "../src";
import fs from "fs-extra";
// Just need a random string for testing, nothing crazy.
const randomStr = () => Date.now().toString(36);

// Make sure to change these values.
const projectId = "bfdb6ae4-bcbf-4738-ad1c-0ff2102721f4";
const environment = "dev";

const MACHINE_IDENTITY_CLIENT_SECRET = "5c943b047536c9fa486153ae6b8e1a3f29f6948c6fc3e06d97a9133195e6235c";
const MACHINE_IDENTITY_CLIENT_ID = "ae652b82-358c-4319-938b-3016db58a960";

process.env = {
	...process.env,

	INFISICAL_UNIVERSAL_AUTH_CLIENT_ID: MACHINE_IDENTITY_CLIENT_ID,
	INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET: MACHINE_IDENTITY_CLIENT_SECRET
};

const client = new InfisicalClient({
	clientId: MACHINE_IDENTITY_CLIENT_ID,
	clientSecret: MACHINE_IDENTITY_CLIENT_SECRET,
	siteUrl: "http://localhost:8080",
	logLevel: LogLevel.Debug // Optional, default is LogLevel.Error.

	// // NEW:
	// auth: {
	// 	universalAuth: {
	// 		clientId: "client_id",
	// 		clientSecret: "client_secret"
	// 	},

	// 	aws: {
	// 		something: "the stuff",
	// 		etc: "etc..."
	// 	}
	// }
});

async function testListSecrets() {
	console.log("⏱️ Listing secrets");

	const secrets = await client.listSecrets({
		projectId,
		recursive: true, // Optional, default is false
		expandSecretReferences: true, // Optional, default is true. This will expand secret references in the response.
		environment
	});

	console.log("\n\n\n\n\n\n\n\n");
	secrets.forEach(sec => console.log(`Secret: ${sec.secretKey}   ---   ${sec.secretValue}`));
	console.log(`Total secrets: ${secrets.length}`);

	if (!secrets.length) {
		throw new Error("❌ testList: secrets.length was 0");
	}

	console.log("✅ Secrets listed");
}

testListSecrets();
