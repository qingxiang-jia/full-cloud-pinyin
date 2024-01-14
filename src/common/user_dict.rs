use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    fs::{create_dir_all, OpenOptions},
    io::{BufRead, BufReader, BufWriter},
    sync::Mutex,
};

use crate::common::path_util::abs_config_path_fcp;

pub struct UserDict {
    filepath: PathBuf,
    dict: Mutex<HashMap<String, String>>,
}

impl Drop for UserDict {
    fn drop(&mut self) {
        self.persist();
    }
}

impl UserDict {
    pub fn new() -> UserDict {
        let config_path = abs_config_path_fcp();
        let ud_path = config_path.join("user_dict.csv");
        // Create folders and files if do not already exist
        let res = create_dir_all(&config_path.as_path());
        if res.is_err() {
            panic!("new: Failed to create missing directories in path.");
        }
        UserDict {
            filepath: ud_path,
            dict: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, preedit: &str, candidate: &str) {
        let mut dict = self.dict.lock().expect("insert: Failed to lock dict.");
        dict.insert(preedit.to_string(), candidate.to_string());
    }

    pub fn get(&self, preedit: &str) -> Option<String> {
        let dict = self.dict.lock().expect("get: Failed to lock dict.");
        let candidate = dict.get(preedit);
        match candidate {
            Some(cand) => Some(cand.to_owned()),
            None => None,
        }
    }

    pub fn load(&self) {
        let res = File::open(&self.filepath);
        if res.is_err() {
            println!("load: Failed to load the file, give up: {:#?}", res.err());
            return;
        }
        let file = res.unwrap();
        let reader = BufReader::new(file);
        let mut map = self.dict.lock().expect("persist: Failed to lock dict.");

        for line in reader.lines() {
            if line.is_err() {
                println!("load: Error reading a line, give up. {:#?}", line.err());
                break;
            }
            let line = line.unwrap();
            let preedit_candidate: Vec<&str> = line.split(",").collect();
            map.insert(
                preedit_candidate[0].to_owned(),
                preedit_candidate[1].to_owned(),
            );
        }
    }

    fn persist(&self) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.filepath)
            .expect("persist: Failed to open the file.");
        let mut writer = BufWriter::new(file);

        let map = &*self.dict.lock().expect("persist: Failed to lock dict.");

        for (preedit, candidate) in map {
            let res = write!(writer, "{},{}\n", preedit, candidate);
            if res.is_err() {
                println!("persist: Failed to write line.");
            }
        }
    }
}
