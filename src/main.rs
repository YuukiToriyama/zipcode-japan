mod constants;
mod ken_all;
mod zip_code;

use crate::zip_code::ZipCode;
use constants::{PUBLISH_DIR, RESOURCE_URL, TEMPORARY_DIR};
use polars::frame::row::Row;
use polars::prelude::IntoLazy;
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::Path;

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
    let path = ken_all::get_csv()
        .await
        .expect("utf_all.csvのダウンロードに失敗しました");
    // DataFrameに変換
    let data_frame = ken_all::read_csv(path);
    // JSONファイルに書き出し
    for index in 0..data_frame.height() {
        let Row(vec) = data_frame.get_row(index).unwrap();
        generate_json(
            vec[0].get_str().unwrap().to_string(),
            vec[1].get_str().unwrap().to_string(),
            vec[2].get_str().unwrap().to_string(),
            vec[3].get_str().unwrap().to_string(),
            vec[4].get_str().unwrap().to_string(),
            vec[5].get_str().unwrap().to_string(),
            vec[6].get_str().unwrap().to_string(),
        );
    }
}

async fn fetch_resource() -> Result<String, reqwest::Error> {
    match reqwest::get(RESOURCE_URL).await {
        Ok(response) => response.text().await,
        Err(_error) => panic!("ファイルを取得できませんでした。 {}", RESOURCE_URL),
    }
}

fn generate_json(
    postal_code: String,
    pref: String,
    pref_kana: String,
    city: String,
    city_kana: String,
    town: String,
    town_kana: String,
) {
    // 郵便番号を前3桁と後4桁に分離する
    let zip_code = ZipCode::new(&postal_code);

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
        "zipCode": postal_code,
        "pref": pref,
        "prefKana": pref_kana,
        "city": city,
        "cityKana": city_kana,
        "town": town,
        "townKana": town_kana,
    });

    // JSONファイルを保存する
    let file_path = format!("{}/{}/{}.json", PUBLISH_DIR, zip_code.pre, zip_code.post);
    let mut file = fs::File::create(&file_path).unwrap();
    match file.write_all(json.to_string().as_bytes()) {
        Ok(_) => println!("🗒️New file was created. {}", &file_path),
        Err(error) => panic!("⚠Error occurs. {}", error),
    };
}
