pub use infisical::client::client_settings::ClientSettings;

use crate::{
    command::Command,
    response::{Response, ResponseIntoString},
};

pub struct Client(infisical::Client);

impl Client {
    pub fn new(settings_input: Option<String>) -> Self {
        let settings = Self::parse_settings(settings_input);
        Self(infisical::Client::new(settings))
    }

    pub async fn run_command(&mut self, input_str: &str) -> String {
        let cmd_value: serde_json::Value = match serde_json::from_str(input_str) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Response::error(format!("Invalid command string: {}", e)).into_string()
            }
        };

        let cmd: Command = match serde_json::from_value(cmd_value) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Response::error(format!("Invalid command value: {}", e)).into_string()
            }
        };

        match cmd {
            // Infisical secrets
            Command::GetSecret(req) => self.0.secrets().get(&req).await.into_string(),
            Command::ListSecrets(req) => self.0.secrets().list(&req).await.into_string(),
            Command::CreateSecret(req) => self.0.secrets().create(&req).await.into_string(),
            Command::UpdateSecret(req) => self.0.secrets().update(&req).await.into_string(),
            Command::DeleteSecret(req) => self.0.secrets().delete(&req).await.into_string(),

            // Symmetric cryptography
            Command::DecryptSymmetric(req) => {
                self.0.cryptography().decrypt_symmetric(&req).into_string()
            }
            Command::EncryptSymmetric(req) => {
                self.0.cryptography().encrypt_symmetric(&req).into_string()
            }
            Command::CreateSymmetricKey(_) => {
                self.0.cryptography().create_symmetric_key().into_string()
            }
        }
    }

    fn parse_settings(settings_input: Option<String>) -> Option<ClientSettings> {
        if let Some(input) = settings_input.as_ref() {
            match serde_json::from_str(input) {
                Ok(settings) => return Some(settings),
                Err(e) => {
                    log::error!("Failed to parse settings: {}", e);
                }
            }
        }
        None
    }
}
