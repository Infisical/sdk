use dotenv::dotenv;
use infisical::{client::client_settings::ClientSettings, Client};

struct Environment {
    client_id: String,
    client_secret: String,
    site_url: String,
}

fn get_environment_variables() -> Environment {
    dotenv().ok();

    let client_id = std::env::var(&"INFISICAL_UNIVERSAL_CLIENT_ID")
        .expect("INFISICAL_UNIVERSAL_CLIENT_ID not found in environment variables.");
    let client_secret = std::env::var(&"INFISICAL_UNIVERSAL_CLIENT_SECRET")
        .expect("INFISICAL_UNIVERSAL_CLIENT_SECRET not found in environment variables.");
    let site_url = std::env::var(&"INFISICAL_SITE_URL")
        .expect("INFISICAL_SITE_URL not found in environment variables.");

    let environment = Environment {
        client_id,
        client_secret,
        site_url,
    };

    return environment;
}

fn create_client() -> Client {
    let environment = get_environment_variables();

    let settings = ClientSettings {
        client_id: Some(environment.client_id),
        client_secret: Some(environment.client_secret),
        access_token: None,
        site_url: Some(environment.site_url),
        cache_ttl: None,
        user_agent: Some("infisical-cryptography-test-sdk".to_string()),
    };

    let client = Client::new(Some(settings));

    return client;
}

#[cfg(test)]
mod tests {

    use infisical::manager::cryptography::{
        decrypt_symmetric::DecryptSymmetricOptions, encrypt_symmetric::EncryptSymmetricOptions,
    };

    use crate::create_client;

    #[tokio::test]
    async fn test_create_key() {
        let mut client = create_client();

        let key = client
            .cryptography()
            .create_symmetric_key()
            .expect("Failed to create key.");

        println!("Key: {}", key.key);

        assert_eq!(key.key.len(), 44); // It should be 44 because its base64 encoded, and 32 bytes long.
    }

    #[tokio::test]
    async fn test_encrypt_symmetric() {
        let mut client = create_client();

        let test_key = &client.cryptography().create_symmetric_key().unwrap(); // We define a static string so the output is predictable and measurable.

        let encrypt_options = EncryptSymmetricOptions {
            key: test_key.key.clone(),
            plaintext: "Infisical".to_string(),
        };

        let encrypted = client
            .cryptography()
            .encrypt_symmetric(&encrypt_options)
            .expect("Failed to encrypt data.");

        assert!(encrypted.ciphertext.len() > 0);
        assert_eq!(encrypted.tag.len(), 24); // It should be 24 because its base64 encoded, and 16 bytes long.
        assert_eq!(encrypted.iv.len(), 16); // It should be 16 because its base64 encoded, and 12 bytes long.
    }

    #[tokio::test]
    async fn test_decrypt_symmetric() {
        let mut client = create_client();
        let plaintext = &"Infisical rocks!".to_string();

        let test_key = &client.cryptography().create_symmetric_key().unwrap(); // We define a static string so the output is predictable and measurable.

        let encrypt_options = EncryptSymmetricOptions {
            key: test_key.key.clone(),
            plaintext: plaintext.clone(),
        };

        let encrypted = client
            .cryptography()
            .encrypt_symmetric(&encrypt_options)
            .expect("Failed to encrypt data.");

        let decrypt_options = DecryptSymmetricOptions {
            key: test_key.key.clone(),
            ciphertext: encrypted.ciphertext,
            iv: encrypted.iv,
            tag: encrypted.tag,
        };

        let decrypted = &client
            .cryptography()
            .decrypt_symmetric(&decrypt_options)
            .expect("Failed to decrypt data.");

        assert_eq!(&decrypted.decrypted, plaintext);
    }
}
