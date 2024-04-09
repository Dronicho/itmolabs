use crate::models::upload::UploadPayload;
use actix_web::rt::time::interval;
use actix_web_lab::{
    sse::{self, Sse},
    util::InfallibleStream,
};
use futures_util::future;
use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<BroadcastClient>,
}

#[derive(Debug, Clone)]
struct BroadcastClient {
    client: mpsc::Sender<sse::Event>,
    id: String,
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();

        let mut ok_clients = Vec::new();

        for client in clients {
            if client
                .client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(
        &self,
        id: String,
    ) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);

        self.inner
            .lock()
            .clients
            .push(BroadcastClient { client: tx, id });

        Sse::from_infallible_receiver(rx)
    }

    pub async fn send_to(&self, msg: UploadPayload, id: String) {
        
        let clients = self.inner.lock().clients.clone();
        let send_futures = clients
            .iter()
            .filter(|client| client.id == id)
            .map(|client| {
                client
                    .client
                    .send(sse::Data::new(serde_json::to_string(&msg).unwrap()).into())
            });

        let _ = future::join_all(send_futures).await;
    }
}
