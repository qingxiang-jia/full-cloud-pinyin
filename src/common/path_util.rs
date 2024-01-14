use std::path::PathBuf;

use dirs::home_dir;

const FCITX5_CONFIG_DIR: &str = ".local/share/fcitx5/";
const PINYIN_DIR: &str = "fcp";
const NEPALI_DIR: &str = "fcn";
const HINDI_DIR: &str = "fch";
const GUJARATI_DIR: &str = "fcg";

pub fn abs_config_path_fcp() -> PathBuf {
    abs_config_path(PINYIN_DIR)
}

pub fn abs_config_path_fcn() -> PathBuf {
    abs_config_path(NEPALI_DIR)
}

pub fn abs_config_path_fch() -> PathBuf {
    abs_config_path(HINDI_DIR)
}

pub fn abs_config_path_fcg() -> PathBuf {
    abs_config_path(GUJARATI_DIR)
}

fn abs_config_path(im_dir: &str) -> PathBuf {
    let mut buf = home_dir().expect("abs_config_path: Failed to get home path.");
    buf = buf.join(FCITX5_CONFIG_DIR);
    buf = buf.join(im_dir);
    buf
}
