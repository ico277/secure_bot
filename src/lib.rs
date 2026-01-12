use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

//#[serde(skip)]

#[derive(Serialize, Deserialize)]
struct BotConfig {

    #[serde(default)]
    pub counter: u128,
}

struct Data {
    config_dir: PathBuf,

    config: Mutex<BotConfig>,
} // User data, which is stored and accessible in all command invocations
impl Data {
    
}