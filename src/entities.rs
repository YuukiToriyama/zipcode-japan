use serde::Deserialize;

#[derive(Deserialize)]
pub struct ZipCodeEntity {
    pub jis_x_0402: String,
    pub old_postal_code: String,
    pub postal_code: String,
    pub pref_kana: String,
    pub city_kana: String,
    pub town_kana: String,
    pub pref: String,
    pub city: String,
    pub town: String,
    pub flag_1: u8,
    pub flag_2: u8,
    pub flag_3: u8,
    pub flag_4: u8,
    pub flag_5: u8,
    pub flag_6: u8,
}
