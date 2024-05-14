use std::num::NonZeroI128;

use clap::{builder::Str, Arg, Args, Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Get(GetArgs),
    Set(SetArgs),
    Remove(RemoveArgs)
}

#[derive(Args)]
struct GetArgs {
    key: Option<String>
}

#[derive(Args)]
struct SetArgs {
    key: Option<String>,
    value: Option<String>
}

#[derive(Args)]
struct RemoveArgs {
    key: Option<String>
}


fn main() {
    let cli = Cli::parse();

    let mut kv_store = KvStore::new();

    match &cli.command {
        Commands::Get(key) => {
             match &key.key {
                 Some(val) => {
                    kv_store.get(val.to_string());
                },
                 None => panic!("key is required")
             }
            
        },
        Commands::Set(args) => {
            
            let key = &args.key;
            let value = &args.value;
            if let (Some(key), Some(value)) = (key, value) {
                kv_store.set(key.to_string(), value.to_string())
            } else {
                panic!("key and value are required")
            }
        },
        Commands::Remove(key) => {
            match &key.key {
                Some(val) => {
                    kv_store.remove(val.to_string())
                },
                None => panic!("key is required")
            }
        }
    }
}