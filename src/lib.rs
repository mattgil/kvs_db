use std::{fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};



pub struct KvStore {
    db_file: File
}
pub type Result<T> = std::result::Result<T, KvStoreError>;

#[derive(Debug, Clone)]
pub struct KvStoreError;

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String},
    Remove {key: String}
}



impl KvStore {
    
    pub fn open( path: &Path) -> Result<KvStore> {
         match File::create(path) {
            Ok(file) => Ok(Self{ db_file: file }),
            Err(err) => panic!("error read file {}", err.to_string())
         }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set { key, value: value };
        let res = serde_json::to_string(&command).and_then(|res| {
            Ok(self.db_file.write_all(res.as_bytes()))
        });
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(KvStoreError)
        }
    }

    pub fn get(&mut self, _key: String) -> Result<Option<String>> {
        panic!("unimplemented")
    }

    pub fn remove(&mut self, _key: String) -> Result<()> {
        panic!("unimplemented")
    }

    
}

