#![allow(dead_code)]
use crate::cursor::{Cursor, Range};
mod cursor;

impl Cursor<'_> {
    fn find_string(&mut self, word: &str) -> bool{
        let chars2 = self.chars.clone();
        let found = chars2.as_str().find(word);
        if let Some(pos) = found {
            self.chars.nth(pos+word.len()-1);
        }
        false
    }

    fn get_till(&mut self, end: &str) -> Range {
        let chars2 = self.chars.clone();
        let start_pos = self.get_pos();
        let found = chars2.as_str().find(end);
        if let Some(pos) = found {
            self.chars.nth(pos-1);
            let end_pos = self.get_pos();
            self.chars.nth(end.len()-1);
            return Range { start: start_pos, end: end_pos};
        }
        Range::none()
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = r#"
    #version 330

    in vec4 v_color;
    out vec4 color;

    void main() {
        color = v_color;
    };"#;
    #[test]
    fn it_works() {
        let mut scanner: Cursor = Cursor::new(TEXT);
        scanner.find_string("out");
        assert_eq!(scanner.get_pos(), 47);

        scanner.eat_whitespace();
        let range = scanner.get_till(";");
        assert_eq!(range.start, 48);
        assert_eq!(range.end, 58);
        println!(
            "range: {:?} - {:?} (len: {})",
            range.start, range.end, range.end - range.start
        );
        let text = &scanner.source[range.start..range.end];
        println!("text: {}", text);
        println!("{:?}", scanner.chars.as_str());
        // println!("{:?}", scanner.chars.as_str());
        assert!(!scanner.is_eof(), "Scanner should not be at end of file.");
        scanner.find_string("};"); //at eol
        assert_eq!(scanner.get_pos(), 110);
        assert!(scanner.is_eof(), "Scanner should be at end of file.");
    }
}
