use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use async_std::{fs, path::{Path, PathBuf}};


pub struct StaticFileHandler {
    inner: Arc<Mutex<StaticFileHandlerInner>>
}

struct StaticFileHandlerInner {
    root_path: PathBuf,
    file_cache: HashMap<PathBuf, Arc<Mutex<String>>>
}

impl StaticFileHandler {
    pub async fn new(root_path: &str) -> std::io::Result<StaticFileHandler> {
        let abs_root_path = fs::canonicalize(root_path).await?;

        Ok(StaticFileHandler {
            inner: Arc::new(Mutex::new(StaticFileHandlerInner {
                root_path : abs_root_path,
                file_cache : Default::default()
            }))
        })
    }

    pub fn clone(&self) -> StaticFileHandler {
        StaticFileHandler {
            inner: self.inner.clone()
        }
    }

    fn get_root_path(&self) -> PathBuf {
        self.inner.lock().unwrap().root_path.clone()
    }

    fn contains(&self, k: &Path) -> bool {
        self.inner.lock().unwrap().file_cache.contains_key(k)
    }

    fn update(&mut self, k: PathBuf, v: String) {
        self.inner.lock().unwrap().file_cache.entry(k).and_modify(|e| *e = Arc::new(Mutex::new(v)));
    }
    
    fn insert(&mut self, k: PathBuf, v: String) {
        self.inner.lock().unwrap().file_cache.insert(k, Arc::new(Mutex::new(v)));
    }

    fn get_cached(&self, k: &Path) -> Option<Arc<Mutex<String>>> {
        match self.inner.lock().unwrap().file_cache.get(k) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }

    async fn check_valid_path(&self, input_path: &Path) -> std::io::Result<bool> {
        let cloned = { self.clone() };

        let abs_input_path = fs::canonicalize(input_path).await?;

        Ok(abs_input_path.as_path().starts_with(&cloned.get_root_path()))
    }

    async fn cache_file(&mut self, input_path: &Path) -> std::io::Result<bool> {
        let mut cloned = { self.clone() };

        debug_assert!(cloned.check_valid_path(input_path).await?);

        // 1. load file
        let new_file_data = fs::read_to_string(input_path).await?;

        // 2. build key
        let abs_input_path = fs::canonicalize(input_path).await?;

        // 3. set key value
        if cloned.contains(&abs_input_path) {
            cloned.update(abs_input_path, new_file_data);
        }
        else {
            cloned.insert(abs_input_path, new_file_data);
        }

        Ok(true)
    }

    async fn get_cached_file(&mut self, input_path: &Path) -> std::io::Result<Arc<Mutex<String>>> {
        let mut cloned = { self.clone() };

        if !self.check_valid_path(input_path).await? {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }

        let abs_input_path = fs::canonicalize(input_path).await?;

        if !cloned.contains(&abs_input_path) {
            cloned.cache_file(&abs_input_path).await?;
            println!("cached the file : {:?}", abs_input_path);
        }
        
        match cloned.get_cached(&abs_input_path) {
            Some(v) => Ok(v.clone()),
            None => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }
    }

    pub async fn get_static_async(&mut self, input_path: &Path) -> std::io::Result<Arc<Mutex<String>>> {
        self.get_cached_file(input_path).await
    }
}

