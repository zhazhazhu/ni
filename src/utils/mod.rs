use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use which::which;

#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct Package {
    pub name: String,
    pub r#type: String,
    pub version: String,
    pub packageManager: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    #[serde(rename = "scripts-info")]
    pub scripts_info: Option<HashMap<String, String>>,
}

pub fn exclude<T: PartialEq + Clone>(arr: &[T], v: T) -> Vec<T> {
    arr.iter().cloned().filter(|item| *item != v).collect()
}

pub fn which_cmd(cmd: &str) -> bool {
    let b = which(cmd);
    match b {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_package_json(path: &str) -> Package {
    let path = Path::new(&path);
    if path.exists() && path.is_file() {
        let file = File::open(&path);
        if let Ok(mut file) = file {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                return match serde_json::from_str::<Package>(&contents) {
                    Ok(v) => v,
                    Err(_) => Package::default(),
                };
            }
            return Package::default();
        }
        return Package::default();
    }
    Package::default()
}
