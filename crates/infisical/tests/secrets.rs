use rand::{distributions::Alphanumeric, Rng};
use std::env;

use infisical::manager::client_secrets::{create_secret, delete_secret};
use infisical::manager::secrets::{CreateSecretOptions, DeleteSecretOptions, Secret};
use infisical::{client::client_settings::ClientSettings, Client};

async fn create_dummy_secret(client: &mut Client) -> Secret {
    let project_id = &get_project_id();
    let environment = &"dev";

    let options = CreateSecretOptions {
        secret_name: random_string(),
        secret_value: random_string(),
        environment: environment.to_string(),
        project_id: project_id.to_string(),
        path: None,
        secret_comment: None,
        r#type: Some("shared".to_string()),
        skip_multiline_encoding: None,
    };

    let secret = create_secret(client, &options).await;

    match secret {
        Ok(secret) => {
            return secret.secret;
        }
        Err(e) => {
            panic!("Failed to create dummy secret: {:?}", e.to_string());
        }
    }
}

async fn delete_dummy_secret(client: &mut Client, secret_name: String) {
    let project_id = &get_project_id();
    let environment = &"dev";

    let options = DeleteSecretOptions {
        secret_name: secret_name.to_string(),
        environment: environment.to_string(),
        project_id: project_id.to_string(),
        path: None,
        r#type: None,
    };

    let secret = delete_secret(client, &options).await;

    match secret {
        Ok(secret) => {
            assert_eq!(secret.secret.environment, environment.as_ref());
            assert_eq!(secret.secret.workspace, project_id.as_ref());
            assert_eq!(secret.secret.secret_key, secret_name.as_ref());
        }
        Err(e) => {
            panic!("Failed to delete dummy secret: {:?}", e.to_string());
        }
    }
}

fn random_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    return s;
}

fn create_client() -> Client {
    let client_id = env::var(&"INFISICAL_UNIVERSAL_CLIENT_ID");
    let client_secret = env::var(&"INFISICAL_UNIVERSAL_CLIENT_SECRET");

    print!("INFISICAL_UNIVERSAL_CLIENT_ID: {:?}", client_id);
    print!("INFISICAL_UNIVERSAL_CLIENT_SECRET: {:?}", client_secret);

    if client_id.is_err() || client_secret.is_err() {
        panic!("INFISICAL_UNIVERSAL_CLIENT_ID or INFISICAL_UNIVERSAL_CLIENT_SECRET not found in environment variables.");
    }

    let settings = ClientSettings {
        client_id: Some(client_id.unwrap()),
        client_secret: Some(client_secret.unwrap()),
        access_token: None,
        site_url: None,
    };

    let client = Client::new(Some(settings));

    return client;
}

fn get_project_id() -> String {
    let project_id = env::var(&"INFISICAL_PROJECT_ID");

    if project_id.is_err() {
        panic!("INFISICAL_PROJECT_ID not found in environment variables.");
    }

    return project_id.unwrap();
}

#[cfg(test)]
mod tests {

    use infisical::manager::{
        client_secrets::{list_secrets, update_secret},
        secrets::{get_secret, GetSecretOptions, ListSecretsOptions, UpdateSecretOptions},
    };

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_create_secret() {
        let mut client = create_client();
        let project_id = &get_project_id();
        let environment = &"dev";

        let secret_name = random_string();
        let secret_value = random_string();

        let options = CreateSecretOptions {
            secret_name: secret_name.to_string(),
            secret_value: secret_value.to_string(),
            environment: environment.to_string(),
            project_id: project_id.to_string(),
            path: None,
            secret_comment: None,
            r#type: Some("shared".to_string()),
            skip_multiline_encoding: None,
        };

        let secret = create_secret(&mut client, &options).await;

        match secret {
            Ok(secret) => {
                assert_eq!(secret.secret.environment, environment.as_ref());
                assert_eq!(secret.secret.workspace, project_id.as_ref());
                assert_eq!(secret.secret.secret_key, secret_name.as_ref());
                assert_eq!(secret.secret.secret_value, secret_value.as_ref());
            }
            Err(e) => {
                panic!("Error: {:?}", e.to_string());
            }
        }

        delete_dummy_secret(&mut client, secret_name).await;
    }

    #[tokio::test]
    async fn test_get_secret() {
        let mut client = create_client();
        let project_id = &get_project_id();
        let environment = &"dev";

        let dummy_secret = create_dummy_secret(&mut client).await;

        let options = GetSecretOptions {
            secret_name: dummy_secret.secret_key.to_string(),
            environment: environment.to_string(),
            project_id: project_id.to_string(),
            path: None,
            r#type: None,
            include_imports: None,
        };

        let secret = get_secret(&mut client, &options).await;

        match secret {
            Ok(secret) => {
                assert_eq!(secret.secret.environment, environment.as_ref());
                assert_eq!(secret.secret.workspace, project_id.as_ref());
                assert_eq!(
                    secret.secret.secret_value,
                    dummy_secret.secret_value.as_ref()
                );
                assert_eq!(secret.secret.secret_key, dummy_secret.secret_key.as_ref());
            }
            Err(e) => {
                println!("Error: {:?}", e.to_string());
                panic!("Error: {:?}", e.to_string());
            }
        }

        delete_dummy_secret(&mut client, dummy_secret.secret_key).await;
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let mut client = create_client();
        let project_id = &get_project_id();
        let environment = &"dev";

        let options = ListSecretsOptions {
            environment: environment.to_string(),
            project_id: project_id.to_string(),
            path: None,
            include_imports: None,
        };

        let dummy_secret = create_dummy_secret(&mut client).await;

        let secrets = list_secrets(&mut client, &options).await;

        match secrets {
            Ok(secrets) => {
                assert!(secrets.secrets.len() > 0);

                let mut found_secret = false;

                // Loop through secrets and make sure they are all in the same environment, and the secret with SECRET_NAME exists
                for secret in secrets.secrets {
                    assert_eq!(secret.environment, environment.as_ref());
                    assert_eq!(secret.workspace, project_id.as_ref());

                    if secret.secret_key == dummy_secret.secret_key {
                        assert_eq!(secret.secret_value, dummy_secret.secret_value);
                        found_secret = true;
                    }
                }

                assert!(found_secret);
            }
            Err(e) => {
                panic!("Error: {:?}", e.to_string());
            }
        }

        delete_dummy_secret(&mut client, dummy_secret.secret_key).await;
    }

    #[tokio::test]
    async fn test_update_secret() {
        let mut client = create_client();
        let project_id = &get_project_id();
        let environment = &"dev";

        let dummy_secret = create_dummy_secret(&mut client).await;

        let options = UpdateSecretOptions {
            secret_name: dummy_secret.secret_key.to_string(),
            secret_value: dummy_secret.secret_value.to_string(),
            environment: environment.to_string(),
            project_id: project_id.to_string(),
            path: None,
            r#type: None,
            skip_multiline_encoding: None,
        };

        let secret = update_secret(&mut client, &options).await;

        match secret {
            Ok(secret) => {
                assert_eq!(secret.secret.environment, environment.as_ref());
                assert_eq!(secret.secret.workspace, project_id.as_ref());
                assert_eq!(
                    secret.secret.secret_value,
                    dummy_secret.secret_value.as_ref()
                );
                assert_eq!(secret.secret.secret_key, dummy_secret.secret_key.as_ref());
            }
            Err(e) => {
                panic!("Error: {:?}", e.to_string());
            }
        }

        delete_dummy_secret(&mut client, dummy_secret.secret_key).await;
    }

    #[tokio::test]
    async fn test_delete_secret() {
        let mut client = create_client();
        let project_id = &get_project_id();
        let environment = &"dev";

        let dummy_secret = create_dummy_secret(&mut client).await;

        let options = DeleteSecretOptions {
            secret_name: dummy_secret.secret_key.to_string(),
            environment: environment.to_string(),
            project_id: project_id.to_string(),
            path: None,
            r#type: None,
        };

        let secret = delete_secret(&mut client, &options).await;

        match secret {
            Ok(secret) => {
                assert_eq!(secret.secret.environment, environment.as_ref());
                assert_eq!(secret.secret.workspace, project_id.as_ref());
                assert_eq!(
                    secret.secret.secret_value,
                    dummy_secret.secret_value.as_ref()
                );
                assert_eq!(secret.secret.secret_key, dummy_secret.secret_key.as_ref());
            }
            Err(e) => {
                panic!("Error: {:?}", e.to_string());
            }
        }
    }
}
