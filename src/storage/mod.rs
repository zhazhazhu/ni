use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::Read,
    io::Write,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub last_run_command: Option<String>,
}

pub type StorageMutex = Mutex<Option<Storage>>;

lazy_static! {
  #[derive(Debug)]
    pub static ref STORAGE: StorageMutex = Mutex::new(None);
    pub static ref CLI_TEMP_PATH: PathBuf = env::temp_dir().join("npack");
    static ref STORAGE_PATH: PathBuf = CLI_TEMP_PATH.join("_storage.json");
}

pub fn load() {
    let mut storage = STORAGE.lock();
    if storage.is_none() {
        if STORAGE_PATH.exists() && STORAGE_PATH.is_file() {
            let file = File::open(STORAGE_PATH.as_path());
            if let Ok(mut file) = file {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    let content = serde_json::from_str::<Storage>(&contents).unwrap();
                    *storage = Some(content);
                }
            }
        } else {
            *storage = Some(Storage {
                last_run_command: None,
            })
        }
    } else {
        *storage = Some(Storage {
            last_run_command: None,
        })
    }
}

pub fn dump(storage: &Storage) -> std::io::Result<()> {
    let path = &STORAGE_PATH;
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let serialized = serde_json::to_string(&storage)?;
    let path = path.to_str().unwrap();

    let file = File::create(path);
    match file {
        Ok(mut file) => {
            file.write_all(serialized.as_bytes())?;
            Ok(())
        }
        Err(err) => {
            println!("{}", err);
            Err(err)
        }
    }
}
