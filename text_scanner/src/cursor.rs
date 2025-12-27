use std::cmp::{max, min};
use std::ops;
use std::str::Chars;

#[derive(Clone)]
pub struct Cursor<'a> {
    pub source: &'a str,
    pub chars: Chars<'a>,
    pub len_text: usize,
    pub newline_found: bool,
    pub is_beginning_of_text: bool,
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start :usize,
    pub end : usize
}

impl Range {
    pub fn none() -> Range {
        Range {
            start: 0,
            end: 0
        }
    }

    pub fn is_none(&self) -> bool {
        self.start == 0 && self.end == 0
    }
}

impl ops::AddAssign for Range {
    fn add_assign(&mut self, rhs: Self) {
        self.start = min(self.start, rhs.start);
        self.end = max(self.end, rhs.end);
    }
}

impl ops::Add for &Range {
    type Output = Range;

    fn add(self, rhs: Self) -> Self::Output {
        Range {
            start: min(self.start, rhs.start),
            end: max(self.end, rhs.end),
        }
    }
}

pub(crate) const EOF_CHAR: char = '\0';

// https://doc.rust-lang.org/beta/nightly-rustc/rustc_lexer/index.html
impl<'a> Cursor<'a> {
    pub fn new (source: &'a str) -> Cursor<'a> {
        Cursor {
            source,
            chars: source.chars(),
            len_text: source.len(),
            newline_found: true, //first line is also a new line!
            is_beginning_of_text: true,
        }
    }

    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn peek_second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn eat_whitespace(&mut self)  {
        self.newline_found = self.is_beginning_of_text; //this will initialy 'reset' the newline_found value to true, since you always start at a newline.
        while self.is_whitespace(self.peek()) {
            let c = self.next();
            if let Some('\n') = c {
                self.newline_found = true;
            }
        }
    }
    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && !self.is_eof() {
            self.next();
        }
    }

    pub fn eat_to_end_of_comment(&mut self) {
        loop {
            if let ('*', '/') = (self.peek(), self.peek_second()) {
                self.next();
                self.next();
                break;
            } else {
                if self.is_eof() {
                    break;
                } else {
                    self.next();
                }
            }
        }
    }
    pub fn eat_to_eol(&mut self) {
        self.eat_while(
            |c| if let '\n' | '\r' = c
            {
                false
            } else {
                true
            }
        );
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
    pub fn get_pos(&mut self) -> usize{
        self.len_text - self.chars.as_str().len()
    }

    pub fn is_whitespace(&self, c: char) -> bool {
        match c {
            // Usual ASCII suspects
            '\u{0009}'   // \t
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space

            // NEXT LINE from latin1
            | '\u{0085}'

            // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK

            // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' => true, // PARAGRAPH SEPARATOR
            '\n' => true,
            _ => false
        }
    }

}
