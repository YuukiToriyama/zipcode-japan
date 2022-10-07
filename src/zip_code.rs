pub struct ZipCode {
    pub pre: String,
    pub post: String,
}

impl ZipCode {
    pub fn new(str: &str) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::zip_code::ZipCode;

    #[test]
    fn it_works() {
        let zip_code = ZipCode::new("0791143");
        assert_eq!(zip_code.pre, "079".to_string());
        assert_eq!(zip_code.post, "1143".to_string());
    }
}
