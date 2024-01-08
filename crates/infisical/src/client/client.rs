use crate::{
    cache::{cache_thread, CachedSecret},
    client::client_settings::ClientSettings,
};
use std::sync::{Arc, Mutex};
pub(crate) struct ClientAuth {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
}
pub struct Client {
    pub(crate) auth: ClientAuth,

    pub(crate) cache: Arc<Mutex<Vec<CachedSecret>>>,
    pub(crate) cache_ttl: u64, // No need for a mutex lock here, as we are only reading this value in the cache thread.

    pub site_url: String,
    pub user_agent: String,
}

impl Client {
    pub fn new(settings_input: Option<ClientSettings>) -> Self {
        let settings = settings_input.unwrap();

        let c = Self {
            auth: ClientAuth {
                client_id: settings.client_id.unwrap_or("".to_string()),
                client_secret: settings.client_secret.unwrap_or("".to_string()),
                access_token: settings.access_token,
            },
            site_url: settings
                .site_url
                .unwrap_or("https://app.infisical.com".to_string()),

            cache: Arc::new(Mutex::new(Vec::new())),
            cache_ttl: settings.cache_ttl.unwrap_or(300),
            user_agent: settings.user_agent.unwrap(),
        };

        if c.cache_ttl != 0 {
            cache_thread(Arc::clone(&c.cache));
        }
        return c;
    }

    pub fn set_cache(&self, new_cache: &[CachedSecret]) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        cache.extend(new_cache.iter().cloned());
    }

    pub fn set_access_token(&mut self, token: String) {
        self.auth.access_token = Some(token);
    }
}
