use std::vec;
use futures::future::join_all;
use xuexi::dictionary::{
    Dictionary,
    Chinese,
    ChineseVersion,
    Laotian
};
use xuexi::word::DetectWord;

mod utils;

#[derive(Default)]
pub struct DictionaryWrapper {
    t_chinese: Option<Dictionary<Chinese>>,
    s_chinese: Option<Dictionary<Chinese>>,
    laotian: Option<Dictionary<Laotian>>
}

#[swift_bridge::bridge]
mod ffi {
    enum Language {
        TraditionalChinese, 
        SimplifiedChinese,
        Laotian
    }

    extern "Rust" {
        type DictionaryWrapper;

        #[swift_bridge(init)]
        fn new() -> DictionaryWrapper;

        async fn load_dictionaries(&mut self);

        fn search_in_dictionaries(&self, lang: Language, sentence: &str) -> Option<String>;
    }
}

impl DictionaryWrapper {
    pub fn new() -> Self {
        DictionaryWrapper::default()
    }

    pub async fn load_dictionaries(&mut self) {
        let cn_fut = vec![
            utils::load_chinese_dictionary(ChineseVersion::Simplified),
            utils::load_chinese_dictionary(ChineseVersion::Traditional),
        ];

        let la = utils::load_laotian_dictionary().await;
        let mut cn_res = join_all(cn_fut).await;

        self.t_chinese = Some(cn_res.pop().unwrap());
        self.s_chinese = Some(cn_res.pop().unwrap());
        self.laotian = Some(la);
    }

    fn search_in_dictionaries(&self, lang: ffi::Language, sentence: &str) -> Option<String> {
        let list = match lang {
            ffi::Language::TraditionalChinese => {
                let cn = self.t_chinese.as_ref().unwrap();
                cn.get_list_detected_words(sentence)
            },
            ffi::Language::SimplifiedChinese => {
                let cn = self.s_chinese.as_ref().unwrap();
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