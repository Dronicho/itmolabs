pub mod command;
pub mod database;

use command::handle_command;
use database::SharedDb;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();
    let db: SharedDb = Arc::new(Mutex::new(HashMap::new()));
    println!("server started on localhost:6379");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("accepting connection");
                let local_db = db.clone();
                tokio::spawn(async move { handler(stream, local_db).await });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handler(mut stream: TcpStream, db: SharedDb) {
    let mut buffer = [0; 1024];
    loop {
        let bytes = stream
            .read(&mut buffer)
            .await
            .expect("failed to read from stream");
        if bytes == 0 {
            return;
        }

        let response = handle_command(
            String::from_utf8_lossy(&buffer[..bytes])
                .split(" ")
                .map(|s| s.to_string())
                .collect(),
            db.clone(),
        )
        .await;

        let resp_str = match response {
            Ok(response) => response,
            Err(e) => format!("Error occurred: {e}"),
        };

        println!("{}", resp_str);

        stream
            .write_all(resp_str.as_bytes())
            .await
            .expect("failed to write to stream");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use database::SharedDb;

    #[tokio::test]
    async fn test_store_and_load() {
        let db: SharedDb = Arc::new(Mutex::new(HashMap::new()));

        let store_command = vec!["store".to_string(), "key".to_string(), "value".to_string()];
        let load_command = vec!["load".to_string(), "key".to_string()];

        let response = handle_command(store_command.clone(), db.clone())
            .await
            .unwrap();
        assert_eq!(response, "Ok");

        let response = handle_command(load_command.clone(), db.clone())
            .await
            .unwrap();
        assert_eq!(response, "value");
    }

    #[tokio::test]
    async fn test_search() {
        let db: SharedDb = Arc::new(Mutex::new(HashMap::new()));

        let store_commands = vec![
            vec!["store".to_string(), "foo".to_string(), "bar".to_string()],
            vec![
                "store".to_string(),
                "foofoo".to_string(),
                "barbar".to_string(),
            ],
            vec!["store".to_string(), "bar".to_string(), "foo".to_string()],
        ];

        for store_command in store_commands {
            let response = handle_command(store_command.clone(), db.clone())
                .await
                .unwrap();
            assert_eq!(response, "Ok");
        }

        let search_command = vec!["search".to_string(), "fo".to_string()];

        let response = handle_command(search_command.clone(), db.clone())
            .await
            .unwrap();
        assert_eq!(response, "foo,foofoo");
    }
}
