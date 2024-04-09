use crate::database::SharedDb;
use anyhow::{anyhow, Ok, Result};

pub async fn handle_command(parts: Vec<String>, db: SharedDb) -> Result<String> {
    match parts.as_slice() {
        [command, args @ ..] => match command.as_str() {
            "store" if args.len() == 2 => {
                let (key, value) = (&args[0], &args[1]);
                let mut db = db.lock().await;
                db.insert(key.to_string(), value.to_string());
                Ok("Ok".to_string())
            }
            "load" if args.len() == 1 => {
                let key = &args[0];
                let db = db.lock().await;
                match db.get(key) {
                    Some(value) => Ok(value.to_string()),
                    None => Err(anyhow!("Key not found")),
                }
            }
            "search" if args.len() == 1 => {
                let term = &args[0];
                let db = db.lock().await;
                Ok(db
                    .keys()
                    .filter(|k| k.starts_with(term))
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(","))
            }
            _ => Ok("Unsupported command or incorrect usage".to_string()),
        },
        _ => Ok("Unsupported command or incorrect usage".to_string()),
    }
}
