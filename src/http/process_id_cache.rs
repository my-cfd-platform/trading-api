use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

pub struct ProcessIdCache {
    cache: Mutex<HashMap<String, String>>,
}

impl ProcessIdCache {
    pub fn new() -> Self {
        return Self {
            cache: Mutex::new(HashMap::new()),
        };
    }

    pub async fn get<T: Serialize + DeserializeOwned>(&self, key: &str) -> Option<T> {
        let cache = self.cache.lock().await;
        return serde_json::from_str(cache.get(key)?).unwrap();
    }

    pub async fn set<T: Serialize + DeserializeOwned>(&self, key: &str, value: T) {
        let mut cache = self.cache.lock().await;
        cache.insert(key.to_string(), serde_json::to_string(&value).unwrap());
    }
}
