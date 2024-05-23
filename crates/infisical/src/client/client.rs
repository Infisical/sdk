use crate::{
    cache::{cache_thread, CachedSecret},
    client::{auth_method_settings::UniversalAuthMethod, client_settings::ClientSettings},
};
use std::sync::{Arc, Mutex};

use super::auth_method_settings::AuthenticationOptions;
pub struct Client {
    pub(crate) auth: AuthenticationOptions,

    pub(crate) cache: Arc<Mutex<Vec<CachedSecret>>>,
    pub(crate) cache_ttl: u64, // No need for a mutex lock here, as we are only reading this value in the cache thread.

    pub site_url: String,
    pub user_agent: String,
}

impl Client {
    pub fn new(settings_input: Option<ClientSettings>) -> Self {
        // We should allow the user to not provide settings, so they can still use encryption methods that don't require authentication.
        let mut settings = settings_input.unwrap_or(ClientSettings::default());

        // Move the deprecated fields to the new auth object for backwards compatibility.
        #[allow(deprecated)]
        {
            settings.auth.access_token = settings.access_token;

            if settings.client_id.is_some() && settings.client_secret.is_some() {
                settings.auth.universal_auth = Some(UniversalAuthMethod {
                    client_id: settings.client_id.unwrap(),
                    client_secret: settings.client_secret.unwrap(),
                });
            }
        }

        let client: Client = Self {
            auth: settings.auth,
            site_url: settings
                .site_url
                .unwrap_or("https://app.infisical.com".to_string()),

            cache: Arc::new(Mutex::new(Vec::new())),
            cache_ttl: settings.cache_ttl.unwrap_or(300),
            user_agent: settings.user_agent.unwrap_or("".to_string()),
        };

        if client.cache_ttl != 0 {
            cache_thread(Arc::clone(&client.cache));
        }
        return client;
    }

    pub fn set_access_token(&mut self, token: String) {
        self.auth.access_token = Some(token);
    }
}
