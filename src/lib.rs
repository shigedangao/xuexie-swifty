#![allow(dead_code)]
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
    laotian: Option<Dictionary<Laotian>>,
    errors: Option<Vec<String>>
}
pub struct CharacterCounter {
    character: String,
    count: i64
}

#[swift_bridge::bridge]
mod ffi {
    enum XuexiLibLanguage {
        Chinese,
        Laotian
    }

    enum XuexiCNVersion {
        Simplified,
        Traditional
    }

    extern "Rust" {
        type DictionaryWrapper;

        #[swift_bridge(init)]
        fn new() -> DictionaryWrapper;

        // Load a chinese dictionary based on the version that the user want
        // The binding does not yet support Option<T> when running in async context
        // 
        // # Arguments
        // 
        // * `&mut self`
        // * `version` - XuexiCNVersion
        async fn load_chinese_dictionary(&mut self, version: XuexiCNVersion);

        // Load a laotian dictionary
        // The binding does not yet support Option<T> when running in async context
        // 
        // # Arguments
        // 
        // * `&mut self`
        async fn load_laotian_dictionary(&mut self);

        // Search a text within the dictionary
        // 
        // # Arguments
        // 
        // * `&self`
        // * `lang` - XuexiLibLanguage
        // * `sentence` - &str
        fn search_in_dictionaries(&self, lang: XuexiLibLanguage, sentence: &str) -> Option<String>;
        
        // Return a vector of errors if there's an error /!\ The binding does not support Option<Vec<T>> yet
        //
        // # Arguments
        //
        // * `&self`
        fn has_errors(&self) -> Vec<String>;
    }

    extern "Rust" {
        type CharacterCounter;

        fn count_character_for_given_sentence(content: &str) -> Vec<CharacterCounter>;

        fn get_character(&self) -> String;

        fn get_count(&self) -> i64;
    }
}

impl DictionaryWrapper {
    pub fn new() -> Self {
        DictionaryWrapper::default()
    }

    pub async fn load_chinese_dictionary(&mut self, version: ffi::XuexiCNVersion) {
        let dictionary = match version {
            ffi::XuexiCNVersion::Simplified => utils::load_chinese_dictionary(ChineseVersion::Simplified).await,
            ffi::XuexiCNVersion::Traditional => utils::load_chinese_dictionary(ChineseVersion::Traditional).await
        };

        let res = match dictionary {
            Ok(dic) => dic,
            Err(err) => {
                if let Some(errs) = self.errors.as_mut() {
                    errs.push(err.to_string());
                }

                return
            }
        };

        self.chinese = Some(res);
    }

    pub async fn load_laotian_dictionary(&mut self) {
        let dictionary = utils::load_laotian_dictionary()
            .await;

        let res = match dictionary {
            Ok(res) => res,
            Err(err) => {
                if let Some(errs) = self.errors.as_mut() {
                    errs.push(err.to_string());
                }

                return
            }
        };
        
        self.laotian = Some(res);
    }

    fn search_in_dictionaries(&self, lang: ffi::XuexiLibLanguage, sentence: &str) -> Option<String> {
        let list = match lang {
            ffi::XuexiLibLanguage::Chinese => {
                let cn = self.chinese.as_ref().expect("Expect to have a traditional chinese dictionary");
                cn.get_list_detected_words(sentence)
            },
            ffi::XuexiLibLanguage::Laotian => {
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

    fn has_errors(&self) -> Vec<String> {
        if let Some(errors) = self.errors.to_owned() {
            return errors
        }

        Vec::new()
    }
}

impl CharacterCounter {
    fn get_character(&self) -> String {
        self.character.clone()
    }

    fn get_count(&self) -> i64 {
        self.count
    }
}

/// Expose the count character for a given string
/// 
/// # Arguments
/// 
/// * `content` - &str
fn count_character_for_given_sentence(content: &str) -> Vec<CharacterCounter> {
    let res = match xuexi::get_character_by_usage(content) {
        Ok(res) => res,
        Err(_) => return Vec::new()
    };
    
    let vec: Vec<_> = res.into_iter().map(|(k, v)| CharacterCounter {
        character: String::from(k),
        count: v
    }).collect();
    
    vec
}