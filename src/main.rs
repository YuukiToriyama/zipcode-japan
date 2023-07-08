mod constants;
mod entities;
mod hwconv;
mod utils;
mod zip_code;

use crate::zip_code::ZipCode;
use constants::RESOURCE_URL;
use entities::ZipCodeEntity;
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::Path;
use utils::{BASE_URL, FILE_NAMES, PUBLISH_DIR, TEMPORARY_DIR};

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
    // 47都道府県について順番に処理していく
    for file_name in FILE_NAMES.iter() {
        let zip_file_path = fetch_archive(file_name).await.unwrap();
        let unzipped_item_path = unzip_archive(&zip_file_path).unwrap();
        // CSVファイルのエンコーディングがShift-JISなのでUTF-8に変換
        let csv_file = fs::read(unzipped_item_path).unwrap();
        let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&csv_file);
        // CSVをパースする
        let mut reader = csv::Reader::from_reader(res.as_bytes());
        for record in reader.records() {
            match record {
                Ok(values) => save_as_json(&values),
                Err(error) => panic!("⚠Error occurs. {}", error),
            }
        }
    }
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

async fn fetch_archive(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 郵便局のサイトからzipファイルをダウンロード
    let url = format!("{}/{}", BASE_URL, file_name);
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(_error) => panic!("[{}] ファイルを取得することができませんでした", file_name),
    };
    // zipファイルを作成
    let file_path = format!("{}/{}", TEMPORARY_DIR, file_name);
    let mut zip_file = match fs::File::create(&file_path) {
        Ok(file) => file,
        Err(_error) => panic!("[{}] ファイルを作成することができませんでした", file_name),
    };
    // zipファイルにデータを書き込む
    let binary = response.bytes().await.unwrap();
    match zip_file.write_all(&binary) {
        Ok(_) => println!("🗒️New file was created. {}", &file_path),
        Err(error) => panic!("⚠Error occurs. {}", error),
    }
    Ok(file_path)
}

fn unzip_archive(file_path: &String) -> Result<String, zip::result::ZipError> {
    // ZipArchiveに読み込ませるために再度ファイルを開く
    let zip_file = fs::File::open(file_path).unwrap();
    // ZipArchiveでzipファイルを展開
    let mut unzipped = match zip::ZipArchive::new(zip_file) {
        Ok(archive) => archive,
        Err(_error) => panic!("[{}] ファイルを展開することができませんでした", file_path),
    };
    let mut extracted_item = unzipped.by_index(0).unwrap();
    let extracted_item_path = extracted_item.enclosed_name().unwrap();
    // 展開されたファイルを保存
    let file_path = format!("{}/{}", TEMPORARY_DIR, extracted_item_path.display());
    let mut out_file = fs::File::create(&file_path).unwrap();
    match std::io::copy(&mut extracted_item, &mut out_file) {
        Ok(_) => println!("🗒️New file was created. {}", &file_path),
        Err(error) => panic!("⚠Error occurs. {}", error),
    }
    Ok(file_path)
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
