use crate::hwconv::HWConv;
use csv::StringRecord;
use serde_json::{json, Value};

pub const BASE_URL: &str = "https://www.post.japanpost.jp/zipcode/dl/kogaki/zip";
pub const FILE_NAMES: [&str; 47] = [
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
pub const TEMPORARY_DIR: &str = "./temp";
pub const PUBLISH_DIR: &str = "./public/v0.3";

pub fn convert_record_to_json(values: &StringRecord) -> Value {
    // 半角カタカナのフィールドを全角カタカナに変換
    let pref_kana = HWConv::new(values.get(3).unwrap());
    let city_kana = HWConv::new(values.get(4).unwrap());
    let town_kana = HWConv::new(values.get(5).unwrap());
    // JSON構造体を作成
    json!({
        "zipCode": values.get(2).unwrap(),
        "pref": values.get(6).unwrap(),
        "prefKana": pref_kana.to_fullwidth(),
        "city": values.get(7).unwrap(),
        "cityKana": city_kana.to_fullwidth(),
        "town": values.get(8).unwrap(),
        "townKana": town_kana.to_fullwidth(),
    })
}

#[test]
fn test_convert_record_to_json() {
    let record = StringRecord::from(vec![
        "01694",
        "08618",
        "0861831",
        "ﾎｯｶｲﾄﾞｳ",
        "ﾒﾅｼｸﾞﾝﾗｳｽﾁｮｳ",
        "ﾌｼﾞﾐﾁｮｳ",
        "北海道",
        "目梨郡羅臼町",
        "富士見町",
        "0",
        "0",
        "0",
        "0",
        "0",
        "0",
    ]);
    let json = json!({
        "zipCode": "0861831",
        "pref": "北海道",
        "prefKana": "ホッカイドウ",
        "city": "目梨郡羅臼町",
        "cityKana": "メナシグンラウスチョウ",
        "town": "富士見町",
        "townKana": "フジミチョウ"
    });
    assert_eq!(convert_record_to_json(&record), json);
}
