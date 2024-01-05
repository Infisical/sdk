import { InfisicalClient } from "../src";

const client = new InfisicalClient({
    clientId: "CLIENT_ID",
    clientSecret: "CLIENT_SECRET",
    siteUrl: "http://localhost:8080",
    cacheTtl: 30
});

(async () => {
    // This key will be used for encryption and decryption. It will be different every time you execute the function.
    const key = await client.createSymmetricKey();

    console.log(`\n\nSymmetric key: ${key}\n\n`);

    const PLAIN_STR = "Infisical is awesome!";

    console.log(`Plain string: ${PLAIN_STR}\n\n`);

    const encrypted = await client.encryptSymmetric({
        plaintext: PLAIN_STR,
        key: key
    });

    console.log(`Encrypted string (b64): ${encrypted.ciphertext}`);
    console.log(`IV (b64): ${encrypted.iv}`);
    console.log(`Tag (b64): ${encrypted.tag}\n\n`);

    const decrypted = await client.decryptSymmetric({
        ciphertext: encrypted.ciphertext,
        key: key,
        iv: encrypted.iv,
        tag: encrypted.tag
    });

    console.log(`Decrypted string: ${decrypted}\n\n`);
})();
