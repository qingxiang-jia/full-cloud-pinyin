use crate::FullCloudPinyin;
use ffi::CandidateWord;

#[cxx::bridge(namespace = "fcp")]
mod ffi {
    struct CandidateWord {
        word: String,
        len: i32,
    }

    extern "Rust" {
        type RustPinyinEngine;

        // Pass in a QuweiEngine ptr, then initialzie a scheduler with that ptr, then fn keyEvent could use scheduler to call Fcitx code
        fn init() -> Box<RustPinyinEngine>;

        fn query_candidates(&self, preedit: &str) -> Vec<CandidateWord>;
    }
}

struct RustPinyinEngine {
    fcpinyin: FullCloudPinyin,
}

fn init() -> Box<RustPinyinEngine> {
    Box::new(RustPinyinEngine {
        fcpinyin: FullCloudPinyin::new(),
    })
}

impl RustPinyinEngine {
    fn query_candidates(&self, preedit: &str) -> Vec<CandidateWord> {
        let candidates = self.fcpinyin.query_candidates(preedit);
        let mut words = Vec::new();

        // There's not need to keep candidates so let's consume it
        for candidate in candidates.into_iter() {
            words.push(CandidateWord {
                word: candidate.word,
                len: candidate.matched_len.unwrap_or_else(|| preedit.len() as i32),
            })
        }

        words
    }
}
