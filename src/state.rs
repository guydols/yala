use crate::models::GroceryList;
use notify::{RecursiveMode, Result as NotifyResult, Watcher};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::fs;
use tokio::sync::broadcast;

const STORE: &'static str = "lists.json";

pub type AppState = Arc<RwLock<HashMap<String, GroceryList>>>;

#[derive(Clone)]
pub struct AppContext {
    pub state: AppState,
    pub update_tx: broadcast::Sender<String>,
}

impl AppContext {
    pub async fn new() -> Self {
        let state = Arc::new(RwLock::new(load_data().await));
        let (update_tx, _) = broadcast::channel(100);

        tokio::spawn(watch_file(update_tx.clone()));

        Self { state, update_tx }
    }
}

async fn load_data() -> HashMap<String, GroceryList> {
    fs::read_to_string(STORE)
        .await
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub async fn save_data(state: &AppState) {
    let data = state.read().unwrap().clone();
    if let Ok(json) = serde_json::to_string_pretty(&data) {
        let _ = fs::write(STORE, json).await;
    }
}

async fn watch_file(tx: broadcast::Sender<String>) {
    let (notify_tx, mut notify_rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = notify::recommended_watcher(move |res: NotifyResult<notify::Event>| {
        if let Ok(event) = res {
            if event.kind.is_modify() {
                let _ = notify_tx.blocking_send(());
            }
        }
    })
    .unwrap();

    let _ = watcher.watch(std::path::Path::new(STORE), RecursiveMode::NonRecursive);

    while let Some(_) = notify_rx.recv().await {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let _ = tx.send(String::new());
    }
}
