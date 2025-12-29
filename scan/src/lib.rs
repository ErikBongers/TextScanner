use std::{fmt, fs};
use text_scanner::TextScanner;

pub type Result<T> = std::result::Result<T, WplError>;

#[derive(Debug)]
pub enum WplError {
    FileError(String),
    ScanError(String),
}

impl fmt::Display for WplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WplError::FileError(msg) => write!(f, "{}", msg),
            WplError::ScanError(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn scan_wpl(path: String) -> Result<Vec<String>> {
    let text = fs::read_to_string(&path).map_err( |err| {
            return WplError::FileError(err.to_string());
        })?;
    let mut scanner = TextScanner::new(&text);
    let mut tracks: Vec<String> = vec![];
    loop {
        if !scanner.find("<media src=\"") {
            break;
        }
        let Ok(first_line) = scanner.get_until("\" ") else {
            return Err(WplError::ScanError("Could not find end of string.".to_string())) //todo get line and pos: todo: add line and pos to scanner? Yes, but calculate it on demand!
        };
        let line = scanner[&first_line].to_string();
        tracks.push(replace_special_xml_chars(&line));
    }
    Ok(tracks)
}

fn replace_special_xml_chars(xml: &str) -> String {
    xml
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = scan_wpl(r#"Z:\Music\My Playlists\Religioso.wpl"#.to_string());

        for line in res.unwrap() {
            println!("{:?}", line);
        }
    }
}