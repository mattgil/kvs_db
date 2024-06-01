use std::{fs::{File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, thread::panicking};

use serde::{Deserialize, Serialize};



pub struct KvStore {
    db_file: File,
   
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
    
    pub fn open( path:  &Path) -> Result<KvStore> {
        let mut final_path = PathBuf::new();
        final_path.push(path);
        final_path.push("log.db");
        let  file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(final_path.as_path());
         match file {
            Ok(file) => Ok(Self{ db_file: file}),
            Err(err) => panic!("error {}",err.to_string())
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

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
                let mut command_list = String::new();
                self.db_file.read_to_string(&mut command_list).unwrap();
                     let result = command_list
                    .lines()
                    .map(|command| {
                        let command: Command = serde_json::from_str(command).unwrap();
                        command
                    }).fold(None,|mut acc, command|{
                        match command {
                           Command::Set {key: k, value} => {
                               if key == k {
                                   acc = Some(value)
                               }
                               acc
                           },
                           Command::Remove {key: k} => {
                            if key == k { 
                                return None
                            }
                            acc
                           }
                        }
                    });
                    
                    Ok(result)
                    
                 
                
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::Remove { key };
        let res = serde_json::to_string(&command).and_then(|op| {
            Ok(self.db_file.write_all((op + "\n").as_bytes()  ))  
        });

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(KvStoreError) 
        }
    }

    
}

