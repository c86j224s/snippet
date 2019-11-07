use std::collections::{HashMap, hash_map};
use std::path::{Path, PathBuf};
use serde_derive::Deserialize;

struct Dirs(HashMap<PathBuf, String>);

impl Dirs {
    pub fn from(base_path: &Path) -> std::io::Result<Dirs> {
        let mut ret = Dirs { 0: Default::default() };

        ret.scan_dir(base_path, base_path)?;

        Ok(ret)
    }

    fn file_hash(&self, file_path: &Path) -> std::io::Result<md5::Digest> {
        let file_data = std::fs::read(file_path.to_path_buf())?;

        Ok(md5::compute(&file_data))
    }

    fn file_hash_str(&self, file_path: &Path) -> std::io::Result<String> {
        Ok(format!("{:x}", self.file_hash(file_path)?))
    }

    fn path_to_pathbuf_lowercase(&self, file_path: &Path) -> PathBuf {
        let lowercase_path_str = file_path.as_os_str().to_string_lossy().to_lowercase();

        Path::new(&lowercase_path_str).to_path_buf()
    }

    fn scan_dir(&mut self, dir_path: &Path, base_path: &Path) -> std::io::Result<()> {
        let meta = std::fs::metadata(dir_path)?;
        if meta.is_dir() {
            for each in std::fs::read_dir(dir_path)? {
                self.scan_dir(&each?.path(), base_path)?;
            }
        }
        else if meta.is_file() {
            let rel_path = dir_path.strip_prefix(base_path).unwrap();

            
            #[cfg(windows)]
            let rel_pathbuf = self.path_to_pathbuf_lowercase(rel_path);
            #[cfg(target_os = "linux")]
            let rel_pathbuf = rel_path.to_path_buf();
            
            self.0.insert(rel_pathbuf, self.file_hash_str(&dir_path)?);
        }

        Ok(())
    }

    pub fn iter(&self) -> hash_map::Iter<PathBuf, String> {
        self.0.iter()
    }
    
    pub fn drain(&mut self) -> hash_map::Drain<PathBuf, String> {
        self.0.drain()
    }

    pub fn get(&self, dir_path: &Path) -> Option<&String> {
        self.0.get(dir_path)
    }

    pub fn remove(&mut self, dir_path: &Path) -> Option<String> {
        self.0.remove(dir_path)
    }
}


#[derive(Deserialize)]
struct Config {
    pub source_path: String,
    pub target_path: String,
    // perforce info..
}

impl Config {
    pub fn from(config_path: &Path) -> std::io::Result<Config> {
        let file_data = std::fs::read_to_string(config_path.to_path_buf())?;

        Ok(toml::from_str(&file_data)?)
    }
}


struct DirDiff {
    source_path: PathBuf,
    target_path: PathBuf,
    pub identical_dirs: HashMap<PathBuf, String>,
    pub added_dirs: HashMap<PathBuf, String>,
    pub changed_dirs: HashMap<PathBuf, String>,
    pub removed_dirs: HashMap<PathBuf, String>
}


impl DirDiff {
    pub fn diff(source_path: &Path, mut source_dirs: Dirs, target_path: &Path, mut target_dirs: Dirs) -> DirDiff {
        let mut ret = DirDiff {
            source_path: source_path.to_path_buf(),
            target_path: target_path.to_path_buf(),
            identical_dirs: Default::default(),
            added_dirs: Default::default(),
            changed_dirs: Default::default(),
            removed_dirs: Default::default()
        };

        for (target_file_path, target_digest) in target_dirs.drain() {
            match source_dirs.remove(&target_file_path) {
                Some(source_digest) => if source_digest == target_digest {
                    ret.identical_dirs.insert(target_file_path, target_digest);
                } else {
                    ret.changed_dirs.insert(target_file_path, target_digest);
                },
                None => {
                    ret.added_dirs.insert(target_file_path, target_digest);
                }
            }
        }

        for (source_file_path, source_digest) in source_dirs.drain() {
            ret.removed_dirs.insert(source_file_path, source_digest);
        }

        ret
    }

    fn copy_file(&self, from_path: &Path, to_path: &Path, file_path: &PathBuf) -> std::io::Result<()> {
        let from = from_path.join(file_path);
        let to = to_path.join(file_path);

        match std::fs::copy(&from, &to) {
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir_all(to.parent().unwrap())?;
                std::fs::copy(&from, &to)?;
            },
            _ => ()
        }

        Ok(())
    }

    fn merge(&self, from_path: &PathBuf, to_path: &PathBuf) -> std::io::Result<()> {
        
        for (file_path, _) in self.added_dirs.iter() {
            self.copy_file(from_path.as_path(), to_path.as_path(), &file_path)?;
        }

        for (file_path, _) in self.changed_dirs.iter() {
            self.copy_file(from_path.as_path(), to_path.as_path(), &file_path)?;
        }

        for (file_path, _) in self.removed_dirs.iter() {
            std::fs::remove_file(to_path.as_path().join(&file_path)).unwrap();    
        }

        Ok(())
    }

    pub fn merge_source_from_target(&self) -> std::io::Result<()> {
        self.merge(&self.target_path, &self.source_path)
    }

    pub fn merge_target_from_source(&self) -> std::io::Result<()> {
        self.merge(&self.source_path, &self.target_path)
    }
}


fn main() {
    let config_path = Path::new("config.toml");
    let config = Config::from(config_path).unwrap();
    let source_path = Path::new(&config.source_path);
    let target_path = Path::new(&config.target_path);

    // scan dirs
    let source_dirs = Dirs::from(source_path).unwrap();
    let target_dirs = Dirs::from(target_path).unwrap();

    let diff = DirDiff::diff(source_path, source_dirs, target_path, target_dirs);
    diff.merge_source_from_target().unwrap();
}

