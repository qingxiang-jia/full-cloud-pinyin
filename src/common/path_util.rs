use std::path::PathBuf;

use dirs::home_dir;

const CONFIG_DIR: &str = ".local/share/fcitx5/fcp/";

pub fn abs_config_path() -> PathBuf {
    let mut buf = home_dir().expect("abs_config_path: Failed to get home path.");
    buf = buf.join(CONFIG_DIR);
    buf
}
