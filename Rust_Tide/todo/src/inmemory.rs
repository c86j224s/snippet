use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::time::Duration;

use async_std::task;

use serde::{Deserialize, Serialize};



#[derive(Default)]
pub struct MessageStorage {
    inner: Arc<Mutex<MessageStorageInner>>
}


#[derive(Default)]
struct MessageStorageInner {
    mm: HashMap<usize, Message>,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    author: Option<String>,
    contents: String
}


impl MessageStorage {
    pub fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }

    pub async fn set_async(&self, id: usize, msg: Message) {
        let contents = &mut self.inner.lock().unwrap().mm;

        if contents.contains_key(&id) {
            contents.entry(id).and_modify(|v| *v = msg);
            return
        }

        contents.insert(id, msg);
    }

    pub async fn get_async(&self, id: usize) -> Option<Message> {
        let contents = &self.inner.lock().unwrap().mm;

        match contents.get(&id) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }

    pub async fn delay(&self) {
        println!("delaying...");
        task::sleep(Duration::from_secs(3)).await;
        println!("done...");
    }

}

