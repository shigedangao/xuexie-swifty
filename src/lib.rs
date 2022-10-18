#![allow(dead_code)]
use crate::dictionary::DictionaryWrapper;
use crate::character::CharacterCounter;

mod utils;
mod dictionary;
mod character;

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

        async fn load_chinese_dictionary(&mut self, version: XuexiCNVersion);

        async fn load_laotian_dictionary(&mut self);

        fn search_in_dictionaries(&self, lang: XuexiLibLanguage, sentence: &str) -> Option<String>;
        
        fn has_errors(&self) -> Vec<String>;

        fn search_and_export(&self, lang: XuexiLibLanguage, sentence: &str) -> Option<String>; 
    }

    extern "Rust" {
        type CharacterCounter;

        fn count_character_for_given_sentence(content: &str) -> Vec<CharacterCounter>;

        fn get_character(&self) -> String;

        fn get_count(&self) -> i64;
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