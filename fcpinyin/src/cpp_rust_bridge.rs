use crate::cpp_rust_bridge::ffi::CandidateWord;
use crate::FullCloudPinyin;

#[cxx::bridge]
mod ffi {
    struct CandidateWord {
        word: String,
        len: i32
    }

    extern "Rust" {
        type RustPinyinEngine;

        fn new() -> Box<RustPinyinEngine>;

        fn get_candidates(&self, preedit: &str, depth: i32) -> Vec<CandidateWord>;
    }
}

struct RustPinyinEngine {
    fcpinyin: FullCloudPinyin
}

fn new() -> Box<RustPinyinEngine> {
    Box::new(RustPinyinEngine {
        fcpinyin: FullCloudPinyin::new()
    })
}

impl RustPinyinEngine {

    fn get_candidates(&self, preedit: &str, depth: i32) -> Vec<CandidateWord> {
        let candidates = self.fcpinyin.get_candidates(preedit, depth);
        let mut words = Vec::new();
        
        // There's not need to keep candidates so let's consume it
        for candidate in candidates.into_iter() {
            words.push(CandidateWord {
                word: candidate.word,
                len: candidate.matched_len.unwrap_or_else(|| -1),
            })
        }

        words
    }
}