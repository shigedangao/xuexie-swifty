use xuexi::dictionary::{ChineseVersion, Dictionary, Chinese, Laotian};

pub async fn load_chinese_dictionary(version: ChineseVersion) -> Result<Dictionary<Chinese>, Box<dyn std::error::Error>> {
    let res = xuexi::load_chinese_dictionary(version)?;

    Ok(res)
}

pub async fn load_laotian_dictionary() -> Result<Dictionary<Laotian>, Box<dyn std::error::Error>> {
    let res = xuexi::load_laotian_dictionary()?;

    Ok(res)
}