use crate::constants::RESOURCE_URL;
use polars::frame::DataFrame;
use polars::prelude::{col, DataType, Field, LazyCsvReader, LazyFileListReader, Schema};
use reqwest::Error;
use std::fs::File;
use std::io;
use std::path::Path;

async fn get_csv() -> Result<&'static Path, Error> {
    let path = Path::new("temp/utf_all.csv");
    let mut file = File::create(path).unwrap();
    let response = reqwest::get(RESOURCE_URL).await?.bytes().await?;
    io::copy(&mut response.as_ref(), &mut file).unwrap();
    Ok(path)
}

fn read_csv(path: &Path) -> DataFrame {
    let scheme = Schema::from_iter(vec![
        Field::new("jisx0402".into(), DataType::String),
        Field::new("old_postal_code".into(), DataType::String),
        Field::new("postal_code".into(), DataType::String),
        Field::new("pref_kana".into(), DataType::String),
        Field::new("city_kana".into(), DataType::String),
        Field::new("town_kana".into(), DataType::String),
        Field::new("pref".into(), DataType::String),
        Field::new("city".into(), DataType::String),
        Field::new("town".into(), DataType::String),
        Field::new("flag1".into(), DataType::String),
        Field::new("flag2".into(), DataType::String),
        Field::new("flag3".into(), DataType::String),
        Field::new("flag4".into(), DataType::String),
        Field::new("flag5".into(), DataType::String),
        Field::new("flag6".into(), DataType::String),
    ]); // https://www.post.japanpost.jp/zipcode/dl/utf-readme.html
    let lazy_frame = LazyCsvReader::new(path)
        .with_has_header(false)
        .with_schema(Some(scheme.into()))
        .finish()
        .unwrap();
    let data_frame = lazy_frame
        .select([
            col("postal_code"),
            col("pref"),
            col("pref_kana"),
            col("city"),
            col("city_kana"),
            col("town"),
            col("town_kana"),
        ])
        .collect()
        .unwrap();
    data_frame
}

#[cfg(test)]
mod tests {
    use crate::ken_all::read_csv;
    use polars::frame::row::Row;
    use polars::prelude::{col, lit, IntoLazy, NamedFrom};

    #[test]
    fn csvの読み込み() {
        let data_frame = read_csv("assets/test.csv".as_ref());
        let data_frame = data_frame
            .clone()
            .lazy()
            .filter(col("postal_code").eq(lit("0640941"))) // 郵便番号が0640941の行のみを抽出
            .select([col("pref"), col("city"), col("town")])
            .collect()
            .unwrap();
        println!("{}", data_frame);
        let row = data_frame.get_row(0).unwrap();
        assert!(row.eq(&Row::new(vec![
            "北海道".into(),
            "札幌市中央区".into(),
            "旭ケ丘".into()
        ])))
    }
}
