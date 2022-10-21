use std::vec;

pub struct HWConv {
    characters: Vec<char>,
}

impl HWConv {
    pub fn new(str: &str) -> Self {
        let characters: Vec<char> = str.chars().collect();
        HWConv { characters }
    }

    pub fn to_fullwidth(self: &Self) -> String {
        let mut characters: Vec<char> = vec![];
        for c in &self.characters {
            let fullwidth_character: char = match c {
                'ｦ' => 'ヲ',
                'ｧ' => 'ァ',
                'ｨ' => 'ィ',
                'ｩ' => 'ゥ',
                'ｪ' => 'ェ',
                'ｫ' => 'ォ',
                'ｬ' => 'ャ',
                'ｭ' => 'ュ',
                'ｮ' => 'ョ',
                'ｯ' => 'ッ',
                'ｱ' => 'ア',
                'ｲ' => 'イ',
                'ｳ' => 'ウ',
                'ｴ' => 'エ',
                'ｵ' => 'オ',
                'ｶ' => 'カ',
                'ｷ' => 'キ',
                'ｸ' => 'ク',
                'ｹ' => 'ケ',
                'ｺ' => 'コ',
                'ｻ' => 'サ',
                'ｼ' => 'シ',
                'ｽ' => 'ス',
                'ｾ' => 'セ',
                'ｿ' => 'ソ',
                'ﾀ' => 'タ',
                'ﾁ' => 'チ',
                'ﾂ' => 'ツ',
                'ﾃ' => 'テ',
                'ﾄ' => 'ト',
                'ﾅ' => 'ナ',
                'ﾆ' => 'ニ',
                'ﾇ' => 'ヌ',
                'ﾈ' => 'ネ',
                'ﾉ' => 'ノ',
                'ﾊ' => 'ハ',
                'ﾋ' => 'ヒ',
                'ﾌ' => 'フ',
                'ﾍ' => 'ヘ',
                'ﾎ' => 'ホ',
                'ﾏ' => 'マ',
                'ﾐ' => 'ミ',
                'ﾑ' => 'ム',
                'ﾒ' => 'メ',
                'ﾓ' => 'モ',
                'ﾔ' => 'ヤ',
                'ﾕ' => 'ユ',
                'ﾖ' => 'ヨ',
                'ﾗ' => 'ラ',
                'ﾘ' => 'リ',
                'ﾙ' => 'ル',
                'ﾚ' => 'レ',
                'ﾛ' => 'ロ',
                'ﾜ' => 'ワ',
                'ﾝ' => 'ン',
                'ﾞ' => {
                    let latest: char = characters.pop().unwrap();
                    match latest {
                        'カ' => 'ガ',
                        'キ' => 'ギ',
                        'ク' => 'グ',
                        'ケ' => 'ゲ',
                        'コ' => 'ゴ',
                        'サ' => 'ザ',
                        'シ' => 'ジ',
                        'ス' => 'ズ',
                        'セ' => 'ゼ',
                        'ソ' => 'ゾ',
                        'タ' => 'ダ',
                        'チ' => 'ヂ',
                        'ツ' => 'ヅ',
                        'テ' => 'デ',
                        'ト' => 'ド',
                        'ハ' => 'バ',
                        'ヒ' => 'ビ',
                        'フ' => 'ブ',
                        'ヘ' => 'ベ',
                        'ホ' => 'ボ',
                        'ウ' => 'ヴ',
                        _ => {
                            characters.push(latest);
                            '゛'
                        }
                    }
                }
                'ﾟ' => {
                    let latest: char = characters.pop().unwrap();
                    match latest {
                        'ハ' => 'パ',
                        'ヒ' => 'ピ',
                        'フ' => 'プ',
                        'ヘ' => 'ペ',
                        'ホ' => 'ポ',
                        _ => {
                            characters.push(latest);
                            '゜'
                        }
                    }
                }
                other => *other,
            };
            characters.push(fullwidth_character);
        }
        characters.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::hwc_conv::HWConv;

    #[test]
    fn new() {
        let hwc = HWConv::new("ｷｮｳﾄﾌ");
        assert_eq!(hwc.characters, vec!['ｷ', 'ｮ', 'ｳ', 'ﾄ', 'ﾌ'])
    }

    #[test]
    fn to_fullwidth() {
        let hwc = HWConv::new("ｷｮｳﾄﾌ");
        assert_eq!(hwc.to_fullwidth(), "キョウトフ");
        let hwc_with_dakuten = HWConv::new("ｷｮｳﾄｼｶﾐｷﾞｮｳｸ");
        assert_eq!(hwc_with_dakuten.to_fullwidth(), "キョウトシカミギョウク");
        let hwc_with_handakuten = HWConv::new("ﾎﾟﾝﾄﾁｮｳ");
        assert_eq!(hwc_with_handakuten.to_fullwidth(), "ポントチョウ");
        let hwc_with_number = HWConv::new("367ｺﾞｳｾﾝ");
        assert_eq!(hwc_with_number.to_fullwidth(), "367ゴウセン");
        let hwc_with_undefined_dakuten = HWConv::new("ﾗﾞﾘﾞﾙﾞﾚﾞﾛﾞ");
        assert_eq!(
            hwc_with_undefined_dakuten.to_fullwidth(),
            "ラ゛リ゛ル゛レ゛ロ゛"
        );
        let hwc_with_undefined_handakuten = HWConv::new("ｱﾟｲﾟｳﾟｴﾟｵﾟ");
        assert_eq!(
            hwc_with_undefined_handakuten.to_fullwidth(),
            "ア゜イ゜ウ゜エ゜オ゜"
        );
    }
}
