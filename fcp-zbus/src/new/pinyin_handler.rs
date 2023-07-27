use std::sync::{Mutex, Arc};

use super::cloud_pinyin_client::CloudPinyinClient;

pub struct PinyinHandler {
    client: Arc<Mutex<CloudPinyinClient>>
}