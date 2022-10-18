use std::collections::BTreeMap;
use xuexi::dictionary::{Dictionary, Chinese, Laotian, ChineseVersion};
use xuexi::word::DetectWord;
use xuexi::definition::Definition;
use xuexi::export::Export;
use super::{ffi, utils};

#[derive(Default)]
pub struct DictionaryWrapper {
    chinese: Option<Dictionary<Chinese>>,
    laotian: Option<Dictionary<Laotian>>,
    errors: Option<Vec<String>>
}

impl DictionaryWrapper {
    pub fn new() -> Self {
        DictionaryWrapper::default()
    }

    /// Load a chinese dictionary based on the version that the user want
    /// The binding does not yet support Option<T> when running in async context
    /// 
    /// # Arguments
    /// 
    /// * `&mut self`
    /// * `version` - XuexiCNVersion
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

    /// Load a laotian dictionary
    /// The binding does not yet support Option<T> when running in async context
    /// 
    /// # Arguments
    /// 
    /// * `&mut self`
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

    /// Search a text within the dictionary
    /// 
    /// # Arguments
    /// 
    /// * `&self`
    /// * `lang` - XuexiLibLanguage
    /// * `sentence` - &str
    pub fn search_in_dictionaries(&self, lang: ffi::XuexiLibLanguage, sentence: &str) -> Option<String> {
        match self.get_definitions(lang, sentence) {
            Some(def) => {
                let json = serde_json::to_string(&def).expect("Expect to convert the list of definitions to JSON");
                Some(json)
            },
            None => None
        }
    }

    // Export the result of the search in dictionaries to CSV
    // /!\ This operation is quite consuming as we recompute the value again. 
    //     But it might be better than passing data from Swift to Rust...
    // 
    // # Arguments
    //
    // * `&self`
    // * `lang` - XuexiLibLanguage
    // * `sentence` - &str
    pub fn search_and_export(&self, lang: ffi::XuexiLibLanguage, sentence: &str) -> Option<String> {
        if let Some(def) = self.get_definitions(lang, sentence) {
            if let Ok(csv) = def.to_csv() {
                return Some(csv); 
            }
        }

        None
    }

    // Return a vector of errors if there's an error /!\ The binding does not support Option<Vec<T>> yet
    //
    // # Arguments
    //
    // * `&self`
    pub fn has_errors(&self) -> Vec<String> {
        if let Some(errors) = self.errors.to_owned() {
            return errors
        }

        Vec::new()
    }

    /// Get the definitions for a target language
    /// 
    /// # Arguments
    /// 
    /// * `&self`
    /// * `lang` - ffi::XuexiLibLanguage
    /// * `sentence` - &str
    fn get_definitions(&self, lang: ffi::XuexiLibLanguage, sentence: &str) -> Option<BTreeMap<String, Definition>> {
        match lang {
            ffi::XuexiLibLanguage::Chinese => {
                let cn = self.chinese.as_ref().expect("Expect to have a traditional chinese dictionary");
                cn.get_list_detected_words(sentence)
            },
            ffi::XuexiLibLanguage::Laotian => {
                let lao = self.laotian.as_ref().expect("Expect to have a laotian dictionary");
                lao.get_list_detected_words(sentence)
            }
        }
    }
}