use std::sync::{Mutex, Arc};

use super::{pinyin_handler::PinyinHandler, symbol_handler::SymbolHandler, number_handler::NumberHandler};

pub struct Dispatcher {
    pinyin_handler: Arc<Mutex<PinyinHandler>>,
    symbol_handler: Arc<Mutex<SymbolHandler>>,
    number_handler: Arc<Mutex<NumberHandler>>,
}