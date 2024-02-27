use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::Mutex;


#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ResponseType{
    String(String),
    None
}

pub struct ProcessIdCache {
    cache: Mutex<HashMap<String, ResponseType>>,
}

impl ProcessIdCache {
    pub fn new() -> Self {
        return Self {
            cache: Mutex::new(HashMap::new()),
        };
    }

    pub async fn get<T: Serialize + DeserializeOwned>(&self, key: &str) -> Option<Option<T>> {
        let cache = self.cache.lock().await;
        let response = cache.get(key)?;

        return match response{
            ResponseType::String(src) => Some(serde_json::from_str(src).unwrap()),
            ResponseType::None => Some(None),
        };
    }

    pub async fn get_and_track_empty<T: Serialize + DeserializeOwned>(&self, key: &str) -> Option<Option<T>> {
        let mut cache = self.cache.lock().await;
        let response = cache.get(key);

        if let None = response{
            cache.insert(key.to_string(), ResponseType::None);
            return None;
        }

        return match response.unwrap(){
            ResponseType::String(src) => Some(serde_json::from_str(src).unwrap()),
            ResponseType::None => Some(None),
        };
    }

    pub async fn set<T: Serialize + DeserializeOwned>(&self, key: &str, value: T) {
        let mut cache = self.cache.lock().await;

        cache.insert(key.to_string(), ResponseType::String(serde_json::to_string(&value).unwrap()));   
    }
}
