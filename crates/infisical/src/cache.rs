use log::{debug, error};

use crate::{manager::secrets::Secret, Client};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, SystemTimeError};

#[derive(Clone)]
pub struct CachedSecret {
    pub key: String,
    pub secret: Secret,

    // unix timestamp
    pub expires_at: u64,
}

fn get_sys_time_in_ms() -> Result<u64, SystemTimeError> {
    let sec = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(e) => return Err(e),
    };

    return Ok(sec * 1000);
}

pub fn create_cache_key(secret_key: &str, secret_type: &str, environment: &str) -> String {
    return format!("{}-{}-{}", secret_key, environment, secret_type);
}

pub fn add_to_cache(client: &mut Client, secret: &Secret) {
    if client.cache_ttl == 0 {
        debug!("[CACHE]: Cache TTL is set to 0, not adding secret to cache.");
        return;
    }

    let key = create_cache_key(&secret.secret_key, &secret.r#type, &secret.environment);

    let existing_secret = get_secret_from_cache(client, &key);

    if existing_secret.is_some() {
        debug!("[CACHE]: Secret already exists in cache, not adding it again.");
        return;
    }

    let expires_at = match get_sys_time_in_ms() {
        Ok(n) => n + (client.cache_ttl * 1000),
        Err(e) => {
            error!("[CACHE]: Error adding secret to cache: {}", e);
            return;
        }
    };

    let cached_secret = CachedSecret {
        key,
        expires_at,
        secret: secret.clone(),
    };

    {
        let mut cache = client.cache.lock().unwrap();
        cache.push(cached_secret);
        debug!(
            "[CACHE]: Element added to cache, index: {:?}",
            cache.len() - 1
        );
    } // Mutex lock guard is dropped here when it goes out of scope
}

pub fn remove_from_cache(
    client: &mut Client,
    secret_key: &str,
    secret_type: &str,
    environment: &str,
) {
    if client.cache_ttl == 0 {
        debug!("[CACHE]: Cache TTL is set to 0, not removing secret from cache.");
        return;
    }

    let key = create_cache_key(&secret_key, &secret_type, &environment);

    let mut cache = client.cache.lock().unwrap();

    if let Some(index) = cache.iter().position(|x| x.key == key) {
        cache.remove(index);
        debug!(
            "[CACHE]: {} removed from cache, removed index: {:?}",
            secret_key, index
        );
    }
}

// We only start this thread if the cache_ttl is greater than 0.
pub fn cache_thread(cache: Arc<Mutex<Vec<CachedSecret>>>) {
    let cloned_cache = Arc::clone(&cache);

    std::thread::spawn(move || loop {
        // We scope it here so it doesn't stay locked while we sleep.
        {
            let mut locked_cache = cloned_cache.lock().unwrap();

            let current_time = match get_sys_time_in_ms() {
                Ok(n) => n,
                Err(e) => {
                    error!("Error getting current time: {}", e);
                    return;
                }
            };

            if let Some(index) = locked_cache
                .iter()
                .position(|x| x.expires_at < current_time)
            {
                locked_cache.remove(index);
                debug!(
                    "[CACHE]: Element removed from cache, removed index: {:?}",
                    index
                );
            }
        }
        // Mutex guard dropped here, allowing other threads to access the cache
        thread::sleep(Duration::from_secs(10)); // Check every 10 seconds, this is an arbitrary number.
    });
}

pub fn get_secret_from_cache(client: &mut Client, key: &String) -> Option<Secret> {
    if client.cache_ttl == 0 {
        debug!("[CACHE]: Cache TTL is set to 0, not adding secret to cache.");
        return None;
    }

    let mut locked_cache = client.cache.lock().unwrap();

    // Get index of the secret
    let index = match locked_cache
        .iter()
        .position(|cached_secret| &cached_secret.key == key)
    {
        Some(index) => index,
        None => return None,
    };

    // Get the new expires at time, if it fails just return no secret.
    let expires_at = match get_sys_time_in_ms() {
        Ok(n) => n + (client.cache_ttl * 1000),
        Err(e) => {
            error!(
                "[CACHE]: Error getting new expiry date for cache element: {}",
                e
            );
            return None;
        }
    };

    let secret = locked_cache[index].secret.clone();

    // Create a new cached secret
    let cached_secret = CachedSecret {
        key: key.to_string(),
        expires_at,
        secret: secret.clone(),
    };

    locked_cache.remove(index); // Remove the old cached secret
    locked_cache.push(cached_secret); // Add the new cached secret

    debug!(
        "[CACHE]: Found cached secret with cache key, and updated the expiry time on it: {}",
        key
    );
    return Some(secret);
}
