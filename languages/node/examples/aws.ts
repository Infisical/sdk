import { InfisicalClient } from "../src";

// Make sure to change these values.
const projectId = "PROJECT_ID";
const environment = "dev";

const uncachedClient = new InfisicalClient({
	siteUrl: "https://c61b724baab4.ngrok.app",
	auth: {
		awsIam: {
			identityId: "e2cddb75-a0e0-4c89-bfc0-4d536599f725"
		}
	}
});

async function main() {
	await testListSecrets();
}

async function testListSecrets() {
	await uncachedClient.listSecrets({
		projectId,
		recursive: true, // Optional, default is false
		expandSecretReferences: true, // Optional, default is true. This will expand secret references in the response.
		environment
	});
}
