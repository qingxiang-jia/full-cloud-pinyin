use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter},
    sync::Mutex,
};

use std::io::Write;

pub struct UserDict {
    filepath: String,
    dict: Mutex<HashMap<String, String>>,
}

impl Drop for UserDict {
    fn drop(&mut self) {
        self.persist();
    }
}

impl UserDict {
    pub fn new() -> UserDict {
        Self::new_with_path("~/.local/share/fcitx5/fcp/user_dict.csv")
    }

    pub fn new_with_path(path: &str) -> UserDict {
        UserDict {
            filepath: path.to_owned(),
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
        let file = File::open(&self.filepath).expect("load: Failed to open file.");
        let reader = BufReader::new(file);
        let mut map = self.dict.lock().expect("persist: Failed to lock dict.");

        for line in reader.lines() {
            if line.is_err() {
                println!("load: Error reading a line, skip.");
                continue;
            }
            let line = line.unwrap();
            let preedit_candidate: Vec<&str> = line.split(",").collect();
            map.insert(
                preedit_candidate[0].to_owned(),
                preedit_candidate[1].to_owned(),
            );
        }
    }

    pub fn persist(&self) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.filepath)
            .expect("persist: Failed to open the file.");
        let mut writer = BufWriter::new(file);

        let map = &*self.dict.lock().expect("persist: Failed to lock dict.");

        for (preedit, candidate) in map {
            let res = write!(writer, "{}, {}\n", preedit, candidate);
            if res.is_err() {
                println!("persist: Failed to write line.");
            }
        }
    }
}
