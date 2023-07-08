mod constants;
mod entities;
mod utils;
mod zip_code;

use crate::zip_code::ZipCode;
use constants::RESOURCE_URL;
use entities::ZipCodeEntity;
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::Path;
use utils::{PUBLISH_DIR, TEMPORARY_DIR};

#[tokio::main]
async fn main() {
    // 一時ファイルを保存するディレクトリを作成
    match fs::create_dir(TEMPORARY_DIR) {
        Ok(_) => println!("📁New directory was created. {}", TEMPORARY_DIR),
        Err(error) => panic!("⚠Error occurs. {}", error),
    }

    // 生成したJSONファイルを保存するディレクトリを作成
    match fs::create_dir_all(PUBLISH_DIR) {
        Ok(_) => println!("📁New directory was created. {}", PUBLISH_DIR),
        Err(error) => panic!("⚠Error occurs. {}", error),
    }

    // CSVを取得
    let csv_string = fetch_resource().await.unwrap();

    // CSVをパースしてJSONとして保存
    parse_csv_and_save_as_json(csv_string);
}

async fn fetch_resource() -> Result<String, reqwest::Error> {
    match reqwest::get(RESOURCE_URL).await {
        Ok(response) => response.text().await,
        Err(_error) => panic!("ファイルを取得できませんでした。 {}", RESOURCE_URL),
    }
}

fn parse_csv_and_save_as_json(csv_string: String) {
    let mut reader = csv::Reader::from_reader(csv_string.as_bytes());
    for record in reader.deserialize() {
        let record: ZipCodeEntity = match record {
            Ok(entity) => entity,
            Err(error) => panic!("⚠Error occurs. {}", error),
        };
        save_as_json(record);
    }
}

fn save_as_json(entity: ZipCodeEntity) {
    // 郵便番号を前3桁と後4桁に分離する
    let zip_code = ZipCode::new(&entity.postal_code);

    // 前3桁でディレクトリを作成
    let target_dir = format!("{}/{}", PUBLISH_DIR, zip_code.pre);
    if !Path::new(&target_dir).exists() {
        match fs::create_dir(target_dir) {
            Ok(_) => println!("📁New directory was created. {}", PUBLISH_DIR),
            Err(error) => panic!("⚠Error occurs. {}", error),
        };
    }

    // JSONを作成
    let json = json!({
        "zipCode": entity.postal_code,
        "pref": entity.pref,
        "prefKana": entity.pref_kana,
        "city": entity.city,
        "cityKana": entity.city_kana,
        "town": entity.town,
        "townKana": entity.town_kana,
    });

    // JSONファイルを保存する
    let file_path = format!("{}/{}/{}.json", PUBLISH_DIR, zip_code.pre, zip_code.post);
    let mut file = fs::File::create(&file_path).unwrap();
    match file.write_all(json.to_string().as_bytes()) {
        Ok(_) => println!("🗒️New file was created. {}", &file_path),
        Err(error) => panic!("⚠Error occurs. {}", error),
    };
}
