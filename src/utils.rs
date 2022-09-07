use xuexi::dictionary::{ChineseVersion, Dictionary, Chinese, Laotian};

pub async fn load_chinese_dictionary(version: ChineseVersion) -> Dictionary<Chinese> {
    xuexi::load_chinese_dictionary(version).expect("Expect to load a Chinese dictionary")
}

pub async fn load_laotian_dictionary() -> Dictionary<Laotian> {
    xuexi::load_laotian_dictionary().expect("Expect to load a Laotian dictionary")
}