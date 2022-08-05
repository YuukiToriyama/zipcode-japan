use serde_json::json;
use std::io::Write;
use std::path::Path;

const BASE_URL: &str = "https://www.post.japanpost.jp/zipcode/dl/kogaki/zip";
const FILE_NAMES: [&str; 47] = [
    "01hokkai.zip", //北海道版郵便番号データ
    "02aomori.zip", //青森県版郵便番号データ
    "03iwate.zip",  //岩手県版郵便番号データ
    "04miyagi.zip", //宮城県版郵便番号データ
    "05akita.zip",  //秋田県版郵便番号データ
    "06yamaga.zip", //山形県版郵便番号データ
    "07fukush.zip", //福島県版郵便番号データ
    "08ibarak.zip", //茨城県版郵便番号データ
    "09tochig.zip", //栃木県版郵便番号データ
    "10gumma.zip",  //群馬県版郵便番号データ
    "11saitam.zip", //埼玉県版郵便番号データ
    "12chiba.zip",  //千葉県版郵便番号データ
    "13tokyo.zip",  //東京都版郵便番号データ
    "14kanaga.zip", //神奈川県版郵便番号データ
    "15niigat.zip", //新潟県版郵便番号データ
    "16toyama.zip", //富山県版郵便番号データ
    "17ishika.zip", //石川県版郵便番号データ
    "18fukui.zip",  //福井県版郵便番号データ
    "19yamana.zip", //山梨県版郵便番号データ
    "20nagano.zip", //長野県版郵便番号データ
    "21gifu.zip",   //岐阜県版郵便番号データ
    "22shizuo.zip", //静岡県版郵便番号データ
    "23aichi.zip",  //愛知県版郵便番号データ
    "24mie.zip",    //三重県版郵便番号データ
    "25shiga.zip",  //滋賀県版郵便番号データ
    "26kyouto.zip", //京都府版郵便番号データ
    "27osaka.zip",  //大阪府版郵便番号データ
    "28hyogo.zip",  //兵庫県版郵便番号データ
    "29nara.zip",   //奈良県版郵便番号データ
    "30wakaya.zip", //和歌山県版郵便番号データ
    "31tottor.zip", //鳥取県版郵便番号データ
    "32shiman.zip", //島根県版郵便番号データ
    "33okayam.zip", //岡山県版郵便番号データ
    "34hirosh.zip", //広島県版郵便番号データ
    "35yamagu.zip", //山口県版郵便番号データ
    "36tokush.zip", //徳島県版郵便番号データ
    "37kagawa.zip", //香川県版郵便番号データ
    "38ehime.zip",  //愛媛県版郵便番号データ
    "39kochi.zip",  //高知県版郵便番号データ
    "40fukuok.zip", //福岡県版郵便番号データ
    "41saga.zip",   //佐賀県版郵便番号データ
    "42nagasa.zip", //長崎県版郵便番号データ
    "43kumamo.zip", //熊本県版郵便番号データ
    "44oita.zip",   //大分県版郵便番号データ
    "45miyaza.zip", //宮崎県版郵便番号データ
    "46kagosh.zip", //鹿児島県版郵便番号データ
    "47okinaw.zip", //沖縄県版郵便番号データ
];
const TEMPORARY_DIR: &str = "./temp";
const PUBLISH_DIR: &str = "./public";

#[tokio::main]
async fn main() {
    // 一時ファイルを保存するディレクトリを作成
    match std::fs::create_dir(TEMPORARY_DIR) {
        Ok(_) => {}
        Err(error) => panic!("{}", error),
    }
    // 生成したJSONファイルを保存するディレクトリを作成
    match std::fs::create_dir(PUBLISH_DIR) {
        Ok(_) => {}
        Err(error) => panic!("{}", error),
    }
    // 47都道府県について順番に処理していく
    for file_name in FILE_NAMES.iter() {
        let downloaded_file_path = fetch_archive(file_name).await.unwrap();
        let extracted_item_path = unzip_archive(&downloaded_file_path).unwrap();
        // CSVファイルのエンコーディングがShift-JISなのでUTF-8に変換
        let csv_file = std::fs::read(extracted_item_path).unwrap();
        let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&csv_file);
        // CSVをパースする
        let mut reader = csv::Reader::from_reader(res.as_bytes());
        for record in reader.records() {
            let values = record.unwrap();
            let json = json!({
                "zipCode": values.get(2).unwrap(),
                "pref": values.get(6).unwrap(),
                "prefKana": values.get(3).unwrap(),
                "city": values.get(7).unwrap(),
                "cityName": values.get(4).unwrap(),
                "town": values.get(8).unwrap(),
                "townName": values.get(5)
            });
            // 郵便番号を前3桁と後4桁に分離する
            let zip_code = ZipCode::new(values.get(2).unwrap());
            // 前3桁でディレクトリを作成
            let target_dir = format!("{}/{}", PUBLISH_DIR, zip_code.pre);
            if !Path::new(&target_dir).exists() {
                std::fs::create_dir(target_dir).unwrap();
            }
            // JSONファイルを保存する
            let file_path = format!("{}/{}/{}.json", PUBLISH_DIR, zip_code.pre, zip_code.post);
            let mut file = std::fs::File::create(file_path).unwrap();
            file.write_all(json.to_string().as_bytes()).unwrap();
        }
    }
}

struct ZipCode {
    pre: String,
    post: String,
}

impl ZipCode {
    fn new(str: &str) -> Self {
        let mut pre = String::new();
        let mut post = String::new();
        for (index, char) in str.chars().enumerate() {
            if index <= 2 {
                pre.push(char);
            } else {
                post.push(char);
            }
        }
        ZipCode { pre, post }
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
    let mut zip_file = match std::fs::File::create(&file_path) {
        Ok(file) => file,
        Err(_error) => panic!("[{}] ファイルを作成することができませんでした", file_name),
    };
    // zipファイルにデータを書き込む
    let binary = response.bytes().await.unwrap();
    zip_file.write_all(&binary).unwrap();
    Ok(file_path)
}

fn unzip_archive(file_path: &String) -> Result<String, zip::result::ZipError> {
    // ZipArchiveに読み込ませるために再度ファイルを開く
    let zip_file = std::fs::File::open(file_path).unwrap();
    // ZipArchiveでzipファイルを展開
    let mut unzipped = match zip::ZipArchive::new(zip_file) {
        Ok(archive) => archive,
        Err(_error) => panic!("[{}] ファイルを展開することができませんでした", file_path),
    };
    let mut extracted_item = unzipped.by_index(0).unwrap();
    let extracted_item_path = extracted_item.enclosed_name().unwrap();
    // 展開されたファイルを保存
    let file_path = format!("{}/{}", TEMPORARY_DIR, extracted_item_path.display());
    let mut out_file = std::fs::File::create(&file_path).unwrap();
    std::io::copy(&mut extracted_item, &mut out_file).unwrap();
    Ok(file_path)
}