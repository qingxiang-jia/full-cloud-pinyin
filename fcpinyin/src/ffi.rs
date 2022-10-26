use crate::FullCloudPinyin;
use ffi::CandidateWord;
use cxx::UniquePtr;

#[cxx::bridge(namespace = "fcp")]
mod ffi {
    struct CandidateWord {
        word: String,
        len: i32,
    }

    extern "Rust" {
        type RustPinyinEngine;

        fn init() -> Box<RustPinyinEngine>;

        fn query_candidates(&self, preedit: &str) -> Vec<CandidateWord>;
    }

    unsafe extern "C++" {
        include!("../fcitx5/src/dummy.h");

        type Dummy;

        fn newDummy() -> UniquePtr<Dummy>;

        fn sayHello(&self);
    }
}

struct RustPinyinEngine {
    fcpinyin: FullCloudPinyin,
    dummy: UniquePtr<ffi::Dummy>
}

fn init() -> Box<RustPinyinEngine> {
    Box::new(RustPinyinEngine {
        fcpinyin: FullCloudPinyin::new(),
        dummy: ffi::newDummy()
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

        self.dummy.sayHello();

        words
    }
}
