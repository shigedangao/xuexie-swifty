#![allow(dead_code)]
use ffi::CNVersion;
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
    chinese: Option<Dictionary<Chinese>>,
    laotian: Option<Dictionary<Laotian>>
}

#[swift_bridge::bridge]
mod ffi {
    enum Language {
        Chinese,
        Laotian
    }

    enum CNVersion {
        Simplified,
        Traditional
    }

    extern "Rust" {
        type DictionaryWrapper;

        #[swift_bridge(init)]
        fn new() -> DictionaryWrapper;

        // Load a chinese dictionary based on the version that the user want
        // 
        // # Arguments
        // 
        // * `&mut self`
        // * `version` - CNVersion
        async fn load_chinese_dictionary(&mut self, version: CNVersion) -> String;

        // Load a laotian dictionary
        // 
        // # Arguments
        // 
        // * `&mut self`
        async fn load_laotian_dictionary(&mut self) -> String;

        // Search a text within the dictionary
        // 
        // # Arguments
        // 
        // * `&self`
        // * `lang` - Language
        // * `sentence` - &str
        fn search_in_dictionaries(&self, lang: Language, sentence: &str) -> Option<String>;
    }
}

impl DictionaryWrapper {
    pub fn new() -> Self {
        DictionaryWrapper::default()
    }

    pub async fn load_chinese_dictionary(&mut self, version: CNVersion) -> String {
        let dictionary = match version {
            CNVersion::Simplified => utils::load_chinese_dictionary(ChineseVersion::Simplified).await,
            CNVersion::Traditional => utils::load_chinese_dictionary(ChineseVersion::Traditional).await
        };

        let res = match dictionary {
            Ok(dic) => dic,
            Err(err) => return err.to_string()
        };

        self.chinese = Some(res);

        String::new()
    }

    pub async fn load_laotian_dictionary(&mut self) -> String {
        let dictionary = utils::load_laotian_dictionary()
            .await;

        let res = match dictionary {
            Ok(res) => res,
            Err(err) => return err.to_string()
        };
        
        self.laotian = Some(res);

        String::new()
    }

    fn search_in_dictionaries(&self, lang: ffi::Language, sentence: &str) -> Option<String> {
        let list = match lang {
            ffi::Language::Chinese => {
                let cn = self.chinese.as_ref().expect("Expect to have a traditional chinese dictionary");
                cn.get_list_detected_words(sentence)
            },
            ffi::Language::Laotian => {
                let lao = self.laotian.as_ref().expect("Expect to have a laotian dictionary");
                lao.get_list_detected_words(sentence)
            }
        };

        match list {
            Some(def) => {
                let json = serde_json::to_string(&def).expect("Expect to convert the list of definitions to JSON");
                Some(json)
            },
            None => None
        }
    }
}