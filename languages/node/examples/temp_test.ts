import { InfisicalClient, ListSecretsOptions, LogLevel } from "../src";
import fs from "fs-extra";
// Just need a random string for testing, nothing crazy.
const randomStr = () => Date.now().toString(36);

// Make sure to change these values.
const projectId = "bfdb6ae4-bcbf-4738-ad1c-0ff2102721f4";
const environment = "dev";

const client = new InfisicalClient({
	logLevel: LogLevel.Debug, // Optional, default is LogLevel.Error.
	auth: {
		awsIam: {
			identityId: "2849825f-08ec-482a-87b6-d786bd8035ff"
		}
	}
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
