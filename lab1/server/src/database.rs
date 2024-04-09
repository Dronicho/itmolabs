use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

// Храним все данные в памяти
pub type SharedDb = Arc<Mutex<HashMap<String, String>>>;
