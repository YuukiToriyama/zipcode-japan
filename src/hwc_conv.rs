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
            let fullwidth_character: char = match Self::halfwidth_to_fullwidth(c) {
                Character::Kana(v) => v,
                Character::Dakuten => Self::dakuon_matcher(&mut characters),
                Character::Handakuten => Self::handakuon_matcher(&mut characters),
                Character::Others(v) => v,
            };
            characters.push(fullwidth_character);
        }
        characters.iter().collect::<String>()
    }

    fn halfwidth_to_fullwidth(c: &char) -> Character {
        match c {
            'ｦ' => Character::Kana('ヲ'),
            'ｧ' => Character::Kana('ァ'),
            'ｨ' => Character::Kana('ィ'),
            'ｩ' => Character::Kana('ゥ'),
            'ｪ' => Character::Kana('ェ'),
            'ｫ' => Character::Kana('ォ'),
            'ｬ' => Character::Kana('ャ'),
            'ｭ' => Character::Kana('ュ'),
            'ｮ' => Character::Kana('ョ'),
            'ｯ' => Character::Kana('ッ'),
            'ｱ' => Character::Kana('ア'),
            'ｲ' => Character::Kana('イ'),
            'ｳ' => Character::Kana('ウ'),
            'ｴ' => Character::Kana('エ'),
            'ｵ' => Character::Kana('オ'),
            'ｶ' => Character::Kana('カ'),
            'ｷ' => Character::Kana('キ'),
            'ｸ' => Character::Kana('ク'),
            'ｹ' => Character::Kana('ケ'),
            'ｺ' => Character::Kana('コ'),
            'ｻ' => Character::Kana('サ'),
            'ｼ' => Character::Kana('シ'),
            'ｽ' => Character::Kana('ス'),
            'ｾ' => Character::Kana('セ'),
            'ｿ' => Character::Kana('ソ'),
            'ﾀ' => Character::Kana('タ'),
            'ﾁ' => Character::Kana('チ'),
            'ﾂ' => Character::Kana('ツ'),
            'ﾃ' => Character::Kana('テ'),
            'ﾄ' => Character::Kana('ト'),
            'ﾅ' => Character::Kana('ナ'),
            'ﾆ' => Character::Kana('ニ'),
            'ﾇ' => Character::Kana('ヌ'),
            'ﾈ' => Character::Kana('ネ'),
            'ﾉ' => Character::Kana('ノ'),
            'ﾊ' => Character::Kana('ハ'),
            'ﾋ' => Character::Kana('ヒ'),
            'ﾌ' => Character::Kana('フ'),
            'ﾍ' => Character::Kana('ヘ'),
            'ﾎ' => Character::Kana('ホ'),
            'ﾏ' => Character::Kana('マ'),
            'ﾐ' => Character::Kana('ミ'),
            'ﾑ' => Character::Kana('ム'),
            'ﾒ' => Character::Kana('メ'),
            'ﾓ' => Character::Kana('モ'),
            'ﾔ' => Character::Kana('ヤ'),
            'ﾕ' => Character::Kana('ユ'),
            'ﾖ' => Character::Kana('ヨ'),
            'ﾗ' => Character::Kana('ラ'),
            'ﾘ' => Character::Kana('リ'),
            'ﾙ' => Character::Kana('ル'),
            'ﾚ' => Character::Kana('レ'),
            'ﾛ' => Character::Kana('ロ'),
            'ﾜ' => Character::Kana('ワ'),
            'ﾝ' => Character::Kana('ン'),
            'ﾞ' => Character::Dakuten,
            'ﾟ' => Character::Handakuten,
            other => Character::Others(other.clone()),
        }
    }

    fn handakuon_matcher(characters: &mut Vec<char>) -> char {
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

    fn dakuon_matcher(characters: &mut Vec<char>) -> char {
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
}

enum Character {
    Kana(char),
    Dakuten,
    Handakuten,
    Others(char),
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
