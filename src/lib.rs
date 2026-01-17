use std::path::PathBuf;

use tokio::sync::Mutex;

//#[serde(skip)]

struct BotConfig {
    pub counter: u128,
}

struct Data {
    config_dir: PathBuf,

    config: Mutex<BotConfig>,
} // User data, which is stored and accessible in all command invocations
impl Data {
    pub async fn get_counter() -> u128 {
        return 0
    }
}