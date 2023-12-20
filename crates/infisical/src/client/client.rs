use crate::client::client_settings::ClientSettings;

pub(crate) struct ClientAuth {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
}
pub struct Client {
    pub(crate) auth: ClientAuth,
    pub site_url: String,
}

impl Client {
    pub fn new(settings_input: Option<ClientSettings>) -> Self {
        let settings = settings_input.unwrap();

        Self {
            auth: ClientAuth {
                client_id: settings.client_id.unwrap_or("".to_string()),
                client_secret: settings.client_secret.unwrap_or("".to_string()),
                access_token: settings.access_token,
            },
            site_url: settings
                .site_url
                .unwrap_or("https://app.infisical.com".to_string()),
        }
    }

    pub fn set_access_token(&mut self, token: String) {
        self.auth.access_token = Some(token);
    }
}
