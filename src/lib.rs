use std::{fs::{File, OpenOptions}, io::{Read, Write}, path::Path};

use serde::{Deserialize, Serialize};



pub struct KvStore {
    db_file: File,
    log: Vec<Command>
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
            let  file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(path);
         match file {
            Ok(mut file) => {
                let mut log = vec![];
                let mut command_list = String::new();
                let result = file.read_to_string(&mut command_list).and_then(|_res| {
                    log = command_list
                    .lines()
                    .map(|command| {
                        let command: Command = serde_json::from_str(command).unwrap();
                        command
                    }).collect();
                    Ok(())
                });

                match result {
                    Ok(_) => Ok(Self{ db_file: file,log}),
                    Err(err) => panic!("error file to string {}",err.to_string())

                }
               
            
            },
            Err(err) => panic!("error read file {}", err.to_string())
         }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set { key, value: value };
        let res = serde_json::to_string(&command).and_then(|res| {
            Ok(self.db_file.write_all((res + "\n").as_bytes()))
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

