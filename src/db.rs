use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type TaskStore = Arc<RwLock<HashMap<Uuid, Task>>>;

pub fn create_store() -> TaskStore {
    Arc::new(RwLock::new(HashMap::new()))
}
