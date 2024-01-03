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
    };

    let client = Client::new(Some(settings));

    return client;
}

#[cfg(test)]
mod tests {

    use std::process;

    use infisical::manager::cryptography::{
        decrypt_symmetric::DecryptSymmetricOptions, encrypt_symmetric::EncryptSymmetricOptions,
    };

    use super::*;

    #[tokio::test]
    async fn test_create_key() {
        let mut client = create_client();

        let key = client
            .cryptography()
            .create_symmetric_key()
            .expect("Failed to create key.");

        println!("Key: {}", key);

        assert_eq!(key.len(), 44); // It should be 44 because its base64 encoded, and 32 bytes long.
    }

    #[tokio::test]
    async fn test_encrypt_symmetric() {
        let mut client = create_client();

        let test_key = &client.cryptography().create_symmetric_key().unwrap(); // We define a static string so the output is predictable and measurable.

        println!("Key: {}", test_key);

        let input = EncryptSymmetricOptions {
            key: test_key.clone(),
            plaintext: "Hello world!".to_string(),
        };

        let encrypted_data = client
            .cryptography()
            .encrypt_symmetric(&input)
            .expect("Failed to encrypt data.");

        println!("Encrypted data: {:?}", encrypted_data);

        let options = DecryptSymmetricOptions {
            key: test_key.clone(),
            iv: encrypted_data.iv,
            tag: encrypted_data.tag,
            ciphertext: encrypted_data.ciphertext,
        };

        let decrypted_string = client
            .cryptography()
            .decrypt_symmetric(&options)
            .expect("Failed to decrypt data.");

        println!("Decrypted string: {}", decrypted_string);
    }
}
