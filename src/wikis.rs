use serde_json::Value;
use surf;
use serde_json;
use md5;

use crate::errors::{self, ApiError};



pub enum FilterMediaType {
    All,
    Audio,
    Image, 
    Video,
}

impl Default for FilterMediaType {
    fn default() -> Self {
        Self::All
    }
}


pub async fn wiktionary_media_links(page_title: &str, filter: FilterMediaType, api_user_agent: String) -> Result<Vec<String>, errors::ApiError> {
    let url = format!("https://en.wiktionary.org/api/rest_v1/page/media-list/{}", page_title);
    let mut response = surf::get(&url)
        .header("Api-User-Agent", api_user_agent)
        .send().await?;

    let status = response.status();
    if status != 200 {
        return Err(ApiError::WikiError { status: status.into(), message: status.canonical_reason().into() })
    }
    let body = response.body_string().await?;
    let v: Value = serde_json::from_str(&body)?;
    
    let items: &Vec<_> = v["items"].as_array().ok_or(ApiError::SerdeParseError(format!("No items found for {}", url)))?;
    
    let files: Vec<String> = items.into_iter()
        .filter(|&item| {
            match filter {
                FilterMediaType::All => true,
                FilterMediaType::Audio => item["type"] == "audio",
                FilterMediaType::Image => item["type"] == "image",
                FilterMediaType::Video => item["type"] == "video"

            }
            //item["type"] == "audio"
        })
        .filter_map(|item| item["title"].as_str())
        .map(|item| {
            //remove wiki prefix
            let mut file_name = item.to_string().replace("File:", "");
            file_name = file_name.replace("Media:", "");
            //First letter has to be uppercase (not always the case) to generate the correct md5 checksum (needed to build the url)
            file_name[0..1].make_ascii_uppercase();
            create_wikimedia_url(&file_name)
        }).collect();

    Ok(files)

}

fn create_wikimedia_url(file_name: &str) -> String {

    let digest = format!("{:x}", md5::compute(file_name));
    let a = digest.chars().nth(0).unwrap();
    let b = digest.chars().nth(1).unwrap();
    

    let url = format!("https://upload.wikimedia.org/wikipedia/commons/{}/{}{}/{}", a, a, b, file_name);
    url
}

#[test]
fn test_media_links () {
    use async_std::task;
    match task::block_on(wiktionary_media_links("apple", FilterMediaType::All, "test".into())) {
        Err(e) => println!("{:?}", e),
        Ok(list) => {
            println!("{:?}", list);
        },
    };
}