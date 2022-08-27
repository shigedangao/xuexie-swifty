use std::thread;
use xuexi::chinese::Dictionary as CNDictionary;
use xuexi::laotian::Dictionary as LaoDictionary;
use xuexi::word::DetectWord;

#[derive(Default)]
pub struct DictionaryWrapper {
    chinese: Option<CNDictionary>,
    laotian: Option<LaoDictionary>
}

#[swift_bridge::bridge]
mod ffi {
    enum Language {
        Chinese,
        Laotian
    }

    extern "Rust" {
        type DictionaryWrapper;

        #[swift_bridge(init)]
        fn new() -> DictionaryWrapper;

        fn load_dictionaries(&mut self);

        fn search_in_dictionaries(&self, lang: Language, sentence: &str) -> Option<String>;
    }
}

impl DictionaryWrapper {
    pub fn new() -> Self {
        DictionaryWrapper::default()
    }

    pub fn load_dictionaries(&mut self) {
        let lao_handle = thread::spawn(|| {
            let mut d = LaoDictionary::new().expect("Expect to create a laotian dictionary");
            d.load();

            d
        });

        let cn_handle = thread::spawn(|| {
            let mut d = CNDictionary::new(None).expect("Expect to create a chinese dictionary");
            d.load().expect("Expect to load cn dictionary");

            d
        });

        let (la, cn) = (
            lao_handle.join().unwrap(),
            cn_handle.join().unwrap()
        );

        self.chinese = Some(cn);
        self.laotian = Some(la);
    }

    fn search_in_dictionaries(&self, lang: ffi::Language, sentence: &str) -> Option<String> {
        let list = match lang {
            ffi::Language::Chinese => {
                let cn = self.chinese.as_ref().unwrap();
                cn.get_list_detected_words(sentence)
            },
            ffi::Language::Laotian => {
                let lao = self.laotian.as_ref().unwrap();
                lao.get_list_detected_words(sentence)
            }
        };

        if let Some(def) = list {
            let json = serde_json::to_string(&def).expect("Expect to convert the list of definitions to JSON");
            return Some(json);
        }

        None
    }
}